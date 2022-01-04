use crate::prelude::place_tile_by_id;
use bevy::math::UVec2;
use bevy::prelude::{Commands, Query};
use bevy_ecs_tilemap::{MapQuery, Tile, TileParent};
use bevy_tileset::prelude::{TileId, TilesetParent, Tilesets};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: Figure out how to handle serialization— ideally in a way that's configurable by the user

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TilesetDTO {
	/// Maps tiles placed with a tileset into a single map
	/// Map ID -> Layer ID -> Position -> Tile ID
	pub tiles: HashMap<u16, HashMap<u16, HashMap<UVec2, TileId>>>,
}

pub fn load_map_dto(
	dto: &TilesetDTO,
	tilesets: &Tilesets,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) {
	for (map_id, layers) in &dto.tiles {
		for (layer_id, positions) in layers.iter() {
			for (position, tile_id) in positions.iter() {
				if let Some(tileset) = tilesets.get_by_id(&tile_id.tileset_id) {
					place_tile_by_id(
						tile_id, tileset, *position, *map_id, *layer_id, commands, map_query,
					);
				}
			}
		}
	}
}

pub fn save_map_dto(
	tiles: &Query<(&Tile, &TileParent, &UVec2, &TilesetParent)>,
	tilesets: &Tilesets,
) -> Option<TilesetDTO> {
	let mut tiles_map = HashMap::new();
	for (tile, parent, pos, tileset) in tiles.iter() {
		let tileset = tilesets.get_by_id(&tileset.0)?;
		let index = tile.texture_index as usize;
		let tile_id = tileset.get_tile_id(&index)?;

		let map = tiles_map
			.entry(parent.map_id)
			.or_insert_with(HashMap::default);
		let layer = map.entry(parent.layer_id).or_insert_with(HashMap::default);
		layer.insert(*pos, *tile_id);
	}
	Some(TilesetDTO { tiles: tiles_map })
}
