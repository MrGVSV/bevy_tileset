pub mod debug;
mod ids;
mod plugin;
mod tileset;

#[cfg(feature = "auto-tile")]
pub mod auto;
pub mod coords;

/// A collection of commonly used modules (import via `bevy_tileset_core::prelude::*`)
pub mod prelude {
	pub use super::ids::{PartialTileId, TileGroupId, TileId, TilesetId};
	pub use super::plugin::TilesetPlugin;
	pub use super::tileset::*;
}
