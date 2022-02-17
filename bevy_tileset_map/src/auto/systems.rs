use crate::auto::traits::{TileInfo, TilemapCache};
use bevy::prelude::{Changed, Commands, Entity, EventReader, Query, With};
use bevy_ecs_tilemap::{GPUAnimated, MapQuery, Tile, TileParent, TilePos};
use bevy_tileset::auto::{AutoTileId, AutoTileRequest, AutoTiler};
use bevy_tileset::prelude::{TileIndex, Tilesets};
use std::cell::RefCell;

/// An event used to notify the system of a removed/replaced auto tile
pub struct RemoveAutoTileEvent {
	pub entity: Entity,
	pub pos: TilePos,
	pub parent: TileParent,
	pub auto_id: AutoTileId,
}

/// __\[SYSTEM\]__ Handles the creation/modification of an auto tile
///
/// This system chooses the appropriate texture based on its surrounding neighbors,
/// and updates any neighbors of the same type in a similar manner
pub(crate) fn on_change_auto_tile(
	mut commands: Commands,
	changed_tiles: Query<
		(Entity, &TilePos, &TileParent, &AutoTileId),
		(Changed<AutoTileId>, With<Tile>),
	>,
	all_tiles: Query<(Entity, &TilePos, &TileParent, &AutoTileId), With<Tile>>,
	mut working_tiles: Query<(
		Entity,
		&TilePos,
		&mut Tile,
		&TileParent,
		&AutoTileId,
		Option<&mut GPUAnimated>,
	)>,
	tilesets: Tilesets,
	map_query: MapQuery,
) {
	// Ensure a change happened
	if changed_tiles.iter().count() < 1 {
		return;
	}

	let mut map_query_cell = RefCell::new(map_query);
	let mut cache = TilemapCache {
		tiles_query: &all_tiles,
		map_query: &map_query_cell,
	};
	let mut tiler = AutoTiler::new(&mut cache);

	for (entity, pos, parent, auto_tile) in changed_tiles.iter() {
		tiler.add_tile(TileInfo::new(entity, pos, parent, auto_tile), true);
	}

	let requests = tiler.finish();

	apply_requests(
		&requests,
		&tilesets,
		&mut working_tiles,
		&mut commands,
		map_query_cell.get_mut(),
	);
}

/// __\[SYSTEM\]__ Handles the removal of auto tiles
///
/// Specifically, notifies the surrounding auto tiles of the change
/// This method needs to be called after the removal but within the same frame, otherwise
/// the query will be empty
pub(crate) fn on_remove_auto_tile(
	mut event: EventReader<RemoveAutoTileEvent>,
	// All tiles (used for the tilemap cache)
	all_tiles: Query<(Entity, &TilePos, &TileParent, &AutoTileId), With<Tile>>,
	mut working_tiles: Query<(
		Entity,
		&TilePos,
		&mut Tile,
		&TileParent,
		&AutoTileId,
		Option<&mut GPUAnimated>,
	)>,
	tilesets: Tilesets,
	map_query: MapQuery,
	mut commands: Commands,
) {
	let mut map_query_cell = RefCell::new(map_query);
	let mut cache = TilemapCache {
		tiles_query: &all_tiles,
		map_query: &map_query_cell,
	};
	let mut tiler = AutoTiler::new(&mut cache);

	for ref evt in event.iter() {
		let RemoveAutoTileEvent {
			entity,
			pos,
			parent,
			auto_id,
		} = evt;
		tiler.add_tile(TileInfo::new(*entity, pos, parent, auto_id), true);
	}

	let requests = tiler.finish();

	apply_requests(
		&requests,
		&tilesets,
		&mut working_tiles,
		&mut commands,
		map_query_cell.get_mut(),
	);
}

/// Applies the given rule requests
fn apply_requests(
	requests: &[AutoTileRequest<TileInfo>],
	tilesets: &Tilesets,
	query: &mut Query<(
		Entity,
		&TilePos,
		&mut Tile,
		&TileParent,
		&AutoTileId,
		Option<&mut GPUAnimated>,
	)>,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) {
	for request in requests.iter() {
		let rule = request.rule;
		let TileInfo { entity, .. } = request.tile;
		if let Ok((.., pos, ref mut tile, parent, auto_tile, ref mut anim)) = query.get_mut(entity)
		{
			if let Some(tileset) = tilesets.get_by_id(&auto_tile.tileset_id) {
				if let Some(tile_name) = tileset.get_tile_name(&auto_tile.group_id) {
					// --- Check If Variant --- //
					let texture_index = tile.texture_index as usize;
					if tileset.is_auto_variant(tile_name, &texture_index, &rule) {
						// The request index is just a variant of the correct state -> skip it
						continue;
					}

					// --- Apply Rule --- //
					if let Some(index) = tileset.get_auto_index(tile_name, rule) {
						match index {
							TileIndex::Standard(idx) => {
								tile.texture_index = idx as u16;

								// Remove animated if it exists
								if anim.is_some() {
									commands.entity(entity).remove::<GPUAnimated>();
								}
							},
							TileIndex::Animated(start, end, speed) => {
								// Even though this texture index isn't seen (due to `GPUAnimated`), we still need to set this
								// so that the system can maintain the same variant across state changes
								tile.texture_index = start as u16;

								if let Some(anim) = anim {
									anim.start = start as u32;
									anim.end = end as u32;
									anim.speed = speed;
								} else {
									commands.entity(entity).insert(GPUAnimated::new(
										start as u32,
										end as u32,
										speed,
									));
								}
							},
						}

						// --- Notify Chunk --- //
						map_query.notify_chunk_for_tile(*pos, parent.map_id, parent.layer_id);
					}
				}
			}
		}
	}
}
