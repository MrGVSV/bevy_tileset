pub use bevy_tileset_tiles as tiles;

pub mod debug;
mod ids;
mod plugin;
mod tileset;

/// A collection of commonly used modules (import via `bevy_tileset::prelude::*`)
pub mod prelude {
	pub use super::ids::{PartialTileId, TileGroupId, TileId, TilesetId};
	pub use super::plugin::TilesetPlugin;
	pub use super::tileset::*;
	pub use bevy_tileset_tiles::prelude::{AnimatedTileData, TileData, TileType};
	#[cfg(feature = "auto-tile")]
	pub use bevy_tileset_tiles::prelude::{AutoTileData, AutoTileRule};
	#[cfg(feature = "variants")]
	pub use bevy_tileset_tiles::prelude::{SimpleTileType, VariantTileData};
}
