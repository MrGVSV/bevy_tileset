pub use bevy_ecs_tileset_tiles as tiles;

#[cfg(feature = "auto-tile")]
pub mod auto;
pub mod debug;
mod ids;
pub mod plugin;
pub mod tileset;

/// A collection of commonly used modules (import via `bevy_ecs_tileset::prelude::*`)
pub mod prelude {
	#[cfg(feature = "auto-tile")]
	pub use super::auto::{AutoTile, RemoveAutoTileEvent};
	pub use super::ids::{PartialTileId, TileGroupId, TileId, TilesetId};
	pub use super::plugin::{TilesetLabel, TilesetPlugin, TilesetStage};
	pub use super::tileset::*;
	pub use bevy_ecs_tileset_tiles::prelude::{AnimatedTileData, TileData, TileType};
	#[cfg(feature = "auto-tile")]
	pub use bevy_ecs_tileset_tiles::prelude::{AutoTileData, AutoTileRule};
	#[cfg(feature = "variants")]
	pub use bevy_ecs_tileset_tiles::prelude::{SimpleTileType, VariantTileData};
}
