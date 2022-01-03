use crate::prelude::{TileId, TilesetParent, Tilesets};
use bevy::math::UVec2;
use bevy::prelude::{Commands, Query};
use bevy_ecs_tilemap::{MapQuery, Tile, TileParent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TilesetDTO {
	pub tiles: HashMap<TileLocation, TileId>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct TileLocation {
	position: UVec2,
	map_id: u16,
	layer_id: u16,
}

pub fn load_map_dto(
	dto: &TilesetDTO,
	tilesets: &Tilesets,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) {
	for (loc, tile_id) in &dto.tiles {
		if let Some(tileset) = tilesets.get_by_id(&tile_id.tileset_id) {
			tileset.place_tile_by_id(
				tile_id,
				loc.position,
				loc.map_id,
				loc.layer_id,
				commands,
				map_query,
			);
		}
	}
}

pub fn save_map_dto(
	tiles: &Query<(&Tile, &TileParent, &UVec2, &TilesetParent)>,
	tilesets: &Tilesets,
) -> TilesetDTO {
	TilesetDTO {
		tiles: tiles
			.iter()
			.map(|(tile, parent, pos, tileset)| {
				let tileset = tilesets.get_by_id(&tileset.0)?;
				let index = tile.texture_index as usize;
				let tile_id = tileset.get_tile_id(&index)?;
				let loc = TileLocation {
					position: *pos,
					map_id: parent.map_id,
					layer_id: parent.layer_id,
				};
				Some((loc, *tile_id))
			})
			.filter_map(|x| x)
			.collect(),
	}
}
