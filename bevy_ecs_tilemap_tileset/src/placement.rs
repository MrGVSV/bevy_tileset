use crate::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;

/// Initialize a tile
///
/// This should only be called when constructing the tilemap (hence the usage of `LayerBuilder`).
/// Generally, the [`place_tile`] method is preferred.
///
/// # Arguments
///
/// * `name`: The name of the tile
/// * `position`: The position of the tile in the map
/// * `commands`: The world `Commands`
/// * `layer_builder`: The layer builder object
///
/// returns: Option<Entity>
///
/// # Examples
///
/// ```
///	let entity = tileset.init_tile(
/// 	"My Tile",
/// 	UVec2::new(5, 5),
/// 	&mut commands,
/// 	&mut layer_builder
/// ).unwrap();
/// ```
pub fn init_tile(
	name: &str,
	tileset: &Tileset,
	position: UVec2,
	commands: &mut Commands,
	layer_builder: &mut LayerBuilder<TileBundle>,
) -> Option<Entity> {
	let id = tileset.get_tile_group_id(name)?;
	init_tile_by_id(id, tileset, position, commands, layer_builder)
}

pub fn init_tile_by_id<TId: Into<PartialTileId>>(
	id: TId,
	tileset: &Tileset,
	position: UVec2,
	commands: &mut Commands,
	layer_builder: &mut LayerBuilder<TileBundle>,
) -> Option<Entity> {
	let id = id.into();
	let (tile_index, tile_data) = tileset.select_tile_by_id(id)?;

	let entity = match tile_index {
		TileIndex::Standard(index) => {
			layer_builder
				.set_tile(
					position,
					Tile {
						texture_index: index as u16,
						..Default::default()
					}
					.into(),
				)
				.ok();
			layer_builder.get_tile_entity(position).ok()?
		}
		TileIndex::Animated(start, end, speed) => {
			layer_builder
				.set_tile(
					position,
					Tile {
						texture_index: start as u16,
						..Default::default()
					}
					.into(),
				)
				.ok();
			let entity = layer_builder.get_tile_entity(position).ok()?;
			commands
				.entity(entity)
				.insert(GPUAnimated::new(start as u32, end as u32, speed));
			entity
		}
	};

	commands.entity(entity).insert(TilesetParent(*tileset.id()));

	#[cfg(feature = "auto-tile")]
	apply_auto_tile(id, tileset, commands, tile_data, entity);

	Some(entity)
}

/// Adds or sets a tile from the tileset
///
/// Automatically handles the tile's [`TileType`].
///
/// If successful, returns an optional tuple containing the old
/// entity (if one existed) and the new entity.
///
/// # Arguments
///
/// * `name`:The name of the tile
/// * `position`: The position of the tile in the map
/// * `map_id`: The map ID
/// * `layer_id`: The layer ID
/// * `commands`: The world `Commands`
/// * `map_query`: The tilemap query object
///
/// returns: Option<Entity>
///
/// # Examples
///
/// ```
/// # use bevy::prelude::{Commands, Res};
/// # use bevy_ecs_tilemap::prelude::MapQuery;
/// # use bevy_tileset::prelude::Tileset;
///
/// fn place_tile(tileset: Res<Tileset>, mut commands: Commands, mut map_query: MapQuery) {
///    	tileset.place_tile(
/// 		"My Tile",
/// 		UVec2::new(10, 10),
/// 		0u16,
/// 		0u16,
/// 		&mut commands,
/// 		&mut map_query
/// 	);
/// }
/// ```
pub fn place_tile(
	name: &str,
	tileset: &Tileset,
	position: UVec2,
	map_id: u16,
	layer_id: u16,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) -> Option<(Option<Entity>, Entity)> {
	let id = tileset.get_tile_group_id(name)?;
	place_tile_by_id(id, tileset, position, map_id, layer_id, commands, map_query)
}

pub fn place_tile_by_id<TId: Into<PartialTileId>>(
	id: TId,
	tileset: &Tileset,
	position: UVec2,
	map_id: u16,
	layer_id: u16,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) -> Option<(Option<Entity>, Entity)> {
	let id = id.into();
	let old = map_query.get_tile_entity(position, map_id, layer_id).ok();
	let (tile_index, tile_data) = tileset.select_tile_by_id(id)?;

	let entity = match tile_index {
		TileIndex::Standard(index) => {
			let entity = map_query
				.set_tile(
					commands,
					position,
					Tile {
						texture_index: index as u16,
						..Default::default()
					},
					map_id,
					layer_id,
				)
				.ok()?;
			commands.entity(entity).remove::<GPUAnimated>().id()
		}
		TileIndex::Animated(start, end, speed) => {
			let entity = map_query
				.set_tile(
					commands,
					position,
					Tile {
						texture_index: start as u16,
						..Default::default()
					},
					map_id,
					layer_id,
				)
				.ok()?;
			commands
				.entity(entity)
				.insert(GPUAnimated::new(start as u32, end as u32, speed))
				.id()
		}
	};

	commands.entity(entity).insert(TilesetParent(*tileset.id()));

	#[cfg(feature = "auto-tile")]
	apply_auto_tile(id, tileset, commands, tile_data, entity);

	map_query.notify_chunk_for_tile(position, map_id, layer_id);

	Some((old, entity))
}

/// Update an entity to match the given tile
///
/// # Arguments
///
/// * `name`: The name of the tile
/// * `entity`: The entity to update
/// * `commands`: The world `Commands`
///
/// returns: Option<Entity>
///
pub fn update_tile(
	name: &str,
	tileset: &Tileset,
	entity: Entity,
	commands: &mut Commands,
) -> Option<Entity> {
	let id = tileset.get_tile_group_id(name)?;
	update_tile_by_id(id, tileset, entity, commands)
}

pub fn update_tile_by_id<TId: Into<PartialTileId>>(
	id: TId,
	tileset: &Tileset,
	entity: Entity,
	commands: &mut Commands,
) -> Option<Entity> {
	let id = id.into();
	let (tile_index, tile_data) = tileset.select_tile_by_id(id)?;

	match tile_index {
		TileIndex::Standard(index) => {
			commands
				.entity(entity)
				.insert(Tile {
					texture_index: index as u16,
					..Default::default()
				})
				.remove::<GPUAnimated>();
		}
		TileIndex::Animated(start, end, speed) => {
			commands
				.entity(entity)
				.insert(Tile {
					texture_index: start as u16,
					..Default::default()
				})
				.insert(GPUAnimated::new(start as u32, end as u32 + 1u32, speed));
		}
	}

	commands.entity(entity).insert(TilesetParent(*tileset.id()));

	#[cfg(feature = "auto-tile")]
	apply_auto_tile(id, tileset, commands, tile_data, entity);

	Some(entity)
}

#[cfg(feature = "auto-tile")]
fn apply_auto_tile<TId: Into<PartialTileId>>(
	id: TId,
	tileset: &Tileset,
	commands: &mut Commands,
	tile_data: &TileData,
	entity: Entity,
) {
	let mut cmds = commands.entity(entity);
	if let TileType::Auto(..) = tile_data.tile() {
		cmds.insert(AutoTile::new(id.into().group_id, *tileset.id()));
	} else {
		cmds.remove::<AutoTile>();
	}
}
