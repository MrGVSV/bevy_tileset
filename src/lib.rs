pub use bevy_ecs_tileset_tiles as tiles;
use serde::{Deserialize, Serialize};

#[cfg(feature = "auto-tile")]
mod auto;
pub mod debug;
pub mod plugin;
pub mod tileset;

/// An ID used to identify a [`Tileset`]
pub type TilesetId = u8;
/// An ID used to identify a tile in a [`Tileset`]
pub type TileGroupId = u32;
/// An ID used to identify a single cell in a tile (i.e. a variant or animation frame)
pub type TileCellId = u16;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct TileId {
	pub cell_id: TileCellId,
	pub group_id: TileGroupId,
	pub tileset_id: TilesetId,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct PartialTileId {
	pub cell_id: TileCellId,
	pub group_id: TileGroupId,
}

impl TileId {
	pub fn eq_variant(&self, other: &TileId) -> bool {
		self == other
	}

	pub fn eq_tile_group(&self, other: &TileId) -> bool {
		self.tileset_id == other.tileset_id && self.group_id == other.group_id
	}

	pub fn eq_tileset(&self, other: &TileId) -> bool {
		self.tileset_id == other.tileset_id
	}

	#[allow(dead_code)]
	pub(crate) fn partial(self) -> PartialTileId {
		PartialTileId {
			cell_id: self.cell_id,
			group_id: self.group_id,
		}
	}
}

impl From<TileId> for (TileCellId, TileGroupId, TilesetId) {
	fn from(id: TileId) -> Self {
		(id.cell_id, id.group_id, id.tileset_id)
	}
}

impl PartialTileId {
	pub fn extend(self, tileset_id: TilesetId) -> TileId {
		TileId {
			cell_id: self.cell_id,
			group_id: self.group_id,
			tileset_id,
		}
	}
}

/// A collection of commonly used modules (import via `bevy_ecs_tileset::prelude::*`)
pub mod prelude {
	#[cfg(feature = "auto-tile")]
	pub use super::auto::{AutoTile, RemoveAutoTileEvent};
	pub use super::plugin::{TilesetLabel, TilesetPlugin, TilesetStage};
	pub use super::tileset::*;
	pub use super::{TileCellId, TileGroupId, TileId, TilesetId};
	pub use bevy_ecs_tileset_tiles::prelude::{AnimatedTileData, TileData, TileType};
	#[cfg(feature = "auto-tile")]
	pub use bevy_ecs_tileset_tiles::prelude::{AutoTileData, AutoTileRule};
	#[cfg(feature = "variants")]
	pub use bevy_ecs_tileset_tiles::prelude::{SimpleTileType, VariantTileData};
}
