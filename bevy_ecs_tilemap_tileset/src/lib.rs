use bevy::math::{IVec2, UVec2};
use bevy_ecs_tilemap::TilePos;

pub use bevy_tileset as tileset;
use bevy_tileset::tileset::coords::TileCoords;

#[cfg(feature = "auto-tile")]
pub(crate) mod auto;
mod placement;
mod plugin;
#[cfg(feature = "serialization")]
mod serialization;

pub mod prelude {
	pub use bevy_tileset::prelude::*;

	#[cfg(feature = "auto-tile")]
	pub use super::auto::RemoveAutoTileEvent;
	pub use super::placement::*;
	pub use super::plugin::{TilesetLabel, TilesetPlugin, TilesetStage};
	#[cfg(feature = "serialization")]
	pub use super::serialization::*;
}

/// The corrdinates of the tile, including the `map_id` and `layer_id`
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(
	feature = "serialization",
	derive(serde::Serialize, serde::Deserialize)
)]
pub struct TileCoord {
	#[cfg_attr(feature = "serialization", serde(with = "TilePosRef"))]
	pub pos: TilePos,
	pub map_id: u16,
	pub layer_id: u16,
}

impl TileCoords for TileCoord {
	fn pos(&self) -> IVec2 {
		let pos: UVec2 = self.pos.into();
		pos.as_ivec2()
	}
}

#[cfg(feature = "serialization")]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "TilePos")]
struct TilePosRef(pub u32, pub u32);
