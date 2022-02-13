//! Tools for serializing and deserializing entire tilemaps with one or more tilesets

use bevy::ecs::system::SystemParam;
use bevy::prelude::Query;
use bevy::utils::{AHashExt, HashMap};
use bevy_ecs_tilemap::{Tile, TileParent, TilePos};
use serde::{Deserialize, Serialize};

use crate::prelude::TilePlacer;
use bevy_tileset::prelude::{TileId, TilesetParent, Tilesets};

/// Contains serializable tilemap data
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct SerializableTile {
	pub id: TileId,
	#[serde(with = "crate::TilePosRef")]
	pub pos: TilePos,
}

/// Contains serializable tilemap data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SerializableTilemap {
	pub data: HashMap<u16, HashMap<u16, Vec<SerializableTile>>>,
}

/// A system parameter that can be used to handle tilemap serialization and deserialization
#[derive(SystemParam)]
pub struct TilemapSerializer<'w, 's> {
	tiles: Query<
		'w,
		's,
		(
			&'static Tile,
			&'static TileParent,
			&'static TilePos,
			&'static TilesetParent,
		),
	>,
	tilesets: Tilesets<'w, 's>,
	tile_placer: TilePlacer<'w, 's>,
}

macro_rules! save_tiles {
	($self: ident, $tile: ident, $parent: ident, $pos: ident, $tileset: ident, $tiles_map: ident) => {
		let tileset = $self.tilesets.get_by_id(&$tileset.0)?;
		let index = $tile.texture_index as usize;
		let tile_id = tileset.get_tile_id(&index)?;
		let map = $tiles_map
			.entry($parent.map_id)
			.or_insert_with(HashMap::default);
		let layer = map.entry($parent.layer_id).or_insert_with(Vec::default);
		let tile = SerializableTile {
			id: *tile_id,
			pos: *$pos,
		};
		layer.push(tile);
	};
}

impl<'w, 's> TilemapSerializer<'w, 's> {
	/// Save all current maps
	pub fn save_maps(&self) -> Option<SerializableTilemap> {
		let capacity = self.tiles.iter().count();
		let mut tiles_map = HashMap::with_capacity(capacity);
		for (tile, parent, pos, tileset) in self.tiles.iter() {
			save_tiles!(self, tile, parent, pos, tileset, tiles_map);
		}
		Some(SerializableTilemap { data: tiles_map })
	}

	/// Save the given map
	pub fn save_map(&self, map_id: u16) -> Option<SerializableTilemap> {
		let mut tiles_map = HashMap::default();
		for (tile, parent, pos, tileset) in self.tiles.iter() {
			if parent.map_id != map_id {
				continue;
			}

			save_tiles!(self, tile, parent, pos, tileset, tiles_map);
		}
		Some(SerializableTilemap { data: tiles_map })
	}

	/// Save the given layer for the given map
	pub fn save_layer(&self, map_id: u16, layer_id: u16) -> Option<SerializableTilemap> {
		let mut tiles_map = HashMap::default();
		for (tile, parent, pos, tileset) in self.tiles.iter() {
			if parent.map_id != map_id || parent.layer_id != layer_id {
				continue;
			}

			save_tiles!(self, tile, parent, pos, tileset, tiles_map);
		}
		Some(SerializableTilemap { data: tiles_map })
	}

	/// Load the given map
	pub fn load_maps(&mut self, tilemap: &SerializableTilemap) {
		for (map_id, layers) in &tilemap.data {
			for (layer_id, tiles) in layers.iter() {
				for tile in tiles {
					self.tile_placer
						.place(tile.id, tile.pos, *map_id, *layer_id)
						.ok();
				}
			}
		}
	}
}
