use crate::auto::auto_tiler::{AutoTiler, TileUpdateRequest};
use crate::auto::AutoTile;
use bevy::prelude::{Changed, Commands, Entity, EventReader, Query, QuerySet, UVec2, With};
use bevy_ecs_tilemap::{GPUAnimated, MapQuery, Tile, TileParent};
use bevy_tileset::prelude::{TileIndex, Tilesets};

/// An event used to notify the system of a removed/replaced auto tile
pub struct RemoveAutoTileEvent(pub Entity);

/// __\[SYSTEM\]__ Handles the creation/modification of an auto tile
///
/// This system chooses the appropriate texture based on its surrounding neighbors,
/// and updates any neighbors of the same type in a similar manner
pub(crate) fn on_change_auto_tile(
	mut commands: Commands,
	mut query: QuerySet<(
		// The newly added tile
		Query<(Entity, &UVec2, &TileParent, &AutoTile), (Changed<AutoTile>, With<Tile>)>,
		// All tiles
		Query<(Entity, &UVec2, &TileParent, &AutoTile), With<Tile>>,
		// Tiles to update
		Query<(
			Entity,
			&UVec2,
			&mut Tile,
			&TileParent,
			&AutoTile,
			Option<&mut GPUAnimated>,
		)>,
	)>,
	tilesets: Tilesets,
	mut map_query: MapQuery,
) {
	// Ensure a change happened
	if query.q0().iter().count() < 1 {
		return;
	}

	let mut tiler = AutoTiler::new(query.q1(), &map_query);

	for (entity, pos, parent, auto_tile) in query.q0().iter() {
		tiler.add_tile(entity, pos, parent, auto_tile, true);
	}

	let requests = tiler.requests();

	apply_requests(
		&requests,
		&tilesets,
		&mut query.q2_mut(),
		&mut commands,
		&mut map_query,
	);
}

/// __\[SYSTEM\]__ Handles the removal of auto tiles
///
/// Specifically, notifies the surrounding auto tiles of the change
/// This method needs to be called after the removal but within the same frame, otherwise
/// the query will be empty
pub(crate) fn on_remove_auto_tile(
	mut event: EventReader<RemoveAutoTileEvent>,
	mut query: QuerySet<(
		// All tiles
		Query<(Entity, &UVec2, &TileParent, &AutoTile), With<Tile>>,
		// Tiles to update
		Query<(
			Entity,
			&UVec2,
			&mut Tile,
			&TileParent,
			&AutoTile,
			Option<&mut GPUAnimated>,
		)>,
	)>,
	tilesets: Tilesets,
	mut map_query: MapQuery,
	mut commands: Commands,
) {
	let mut tiler = AutoTiler::new(query.q0(), &map_query);

	for RemoveAutoTileEvent(entity) in event.iter() {
		if let Ok((entity, pos, parent, auto_tile)) = query.q0().get(*entity) {
			tiler.add_tile(entity, pos, parent, auto_tile, false);
		}
	}

	let requests = tiler.requests();

	apply_requests(
		&requests,
		&tilesets,
		&mut query.q1_mut(),
		&mut commands,
		&mut map_query,
	);
}

/// Applies the given rule requests
fn apply_requests(
	requests: &[TileUpdateRequest],
	tilesets: &Tilesets,
	query: &mut Query<(
		Entity,
		&UVec2,
		&mut Tile,
		&TileParent,
		&AutoTile,
		Option<&mut GPUAnimated>,
	)>,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) {
	for TileUpdateRequest { entity, rule } in requests.iter() {
		if let Ok((.., pos, ref mut tile, parent, auto_tile, ref mut anim)) = query.get_mut(*entity)
		{
			if let Some(tileset) = tilesets.get_by_id(auto_tile.tileset_id()) {
				if let Some(tile_name) = tileset.get_tile_name(auto_tile.id()) {
					// --- Check If Variant --- //
					let texture_index = tile.texture_index as usize;
					if tileset.is_auto_variant(tile_name, &texture_index, rule) {
						// The request index is just a variant of the correct state -> skip it
						continue;
					}

					// --- Apply Rule --- //
					if let Some(index) = tileset.get_auto_index(tile_name, *rule) {
						match index {
							TileIndex::Standard(idx) => {
								tile.texture_index = idx as u16;

								// Remove animated if it exists
								if anim.is_some() {
									commands.entity(*entity).remove::<GPUAnimated>();
								}
							}
							TileIndex::Animated(start, end, speed) => {
								// Even though this texture index isn't seen (due to `GPUAnimated`), we still need to set this
								// so that the system can maintain the same variant across state changes
								tile.texture_index = start as u16;

								if let Some(anim) = anim {
									anim.start = start as u32;
									anim.end = end as u32;
									anim.speed = speed;
								} else {
									commands.entity(*entity).insert(GPUAnimated::new(
										start as u32,
										end as u32,
										speed,
									));
								}
							}
						}

						// --- Notify Chunk --- //
						map_query.notify_chunk_for_tile(*pos, parent.map_id, parent.layer_id);
					}
				}
			}
		}
	}
}
