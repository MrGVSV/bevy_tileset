//! Types and tools for handling auto tiling

use bevy::prelude::Component;

pub use auto_tiler::AutoTiler;
pub use traits::{AutoTile, AutoTileRequest, AutoTilemap, TileCoords};

use crate::ids::{TileGroupId, TileId, TilesetId};

mod auto_tiler;
mod traits;

/// A component used to ID an Auto Tile
///
/// This should be attached to every tile that wishes to participate in some type of auto tiling
#[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
pub struct AutoTileId {
	pub group_id: TileGroupId,
	pub tileset_id: TilesetId,
}

impl From<TileId> for AutoTileId {
	fn from(id: TileId) -> Self {
		Self {
			group_id: id.group_id,
			tileset_id: id.tileset_id,
		}
	}
}

impl From<AutoTileId> for TileId {
	fn from(id: AutoTileId) -> Self {
		Self::new(id.group_id, id.tileset_id)
	}
}
