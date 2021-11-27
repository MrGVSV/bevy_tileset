pub use tileset::{TileIndex, Tileset, Tilesets};

pub mod debug;
mod handles;
pub mod plugin;
pub mod tiles;
pub mod tileset;

/// An ID used to identify a [`Tileset`]
pub type TilesetId = u8;
/// An ID used to identify a tile in a [`Tileset`]
pub type TileId = u32;

/// A collection of commonly used modules (import via `bevy_ecs_tilemap_tileset::prelude::*`)
pub mod prelude {
	pub use super::plugin::*;
	pub use super::tiles::*;
	pub use super::tileset::*;
	pub use super::{TileId, TilesetId};
}
