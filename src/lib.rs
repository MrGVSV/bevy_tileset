//! # bevy_tileset
//!
//! > Simple, configurable tilesets in Bevy using RON.
//!
//! This crate provides a way for tilesets to be constructed at runtime either
//! manually in the code or automatically with configuration files.
//!
//! ## Usaage
//!
//! Simply __define__ your tiles and tilesets in config files:
//!
//! ```ron
//! // assets/tiles/my_tile.ron
//! (
//!   name: "My Tile",
//!   tile: Standard("textures/my_tile.png")
//! )
//! ```
//!
//! ```ron
//! // assets/my_tileset.ron
//! (
//!   name: Some("My Awesome Tileset"),
//!   id: 0,
//!   tiles: {
//!     0: "../tiles/my_tile.ron",
//!     // ...
//!   }
//! )
//! ```
//!
//! And __load__ it in via a system:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_tileset::prelude::*;
//!
//! fn load_tiles(asset_server: Res<AssetServer>) {
//!   let handle: Handle<Tileset> = asset_server.load("my_tileset.ron");
//!   // Store handle...
//! }
//! ```
//!
//! Then __access__ the generated tileset from anywhere:
//!
//! ```
//! use bevy_tileset::prelude::*;
//! fn my_system(tilesets: Tilesets, /* other system params */) {
//!
//!   let tileset = tilesets.get_by_name("My Awesome Tileset").unwrap();
//!   let tile_index = tileset.get_tile_index("My Tile").unwrap();
//!
//!   match tile_index {
//!     TileIndex::Standard(texture_index) => { /* Do something */ },
//!     TileIndex::Animated(start, end, speed) => { /* Do something */ },
//!   }
//! }
//! ```
//!
//! ## Crate Features
//!
//! * __`default`__ - No features automatically enabled
//! * __`variants`__ - Enables usage of Variant tiles
//! * __`auto-tile`__ - Enables usage of Auto tiles
//!

/// A re-export of `bevy_tileset_core` in case non-prelude modules are needed
pub use bevy_tileset_core as tileset;
/// A re-export of `bevy_tileset_tiles` in case non-prelude modules are needed
pub use bevy_tileset_tiles as tiles;

/// The main module to import when using this crate
///
/// # Examples
///
/// ```
/// use bevy_tileset::prelude::*;
/// ```
pub mod prelude {
	pub use bevy_tileset_core::prelude::*;
	pub use bevy_tileset_tiles::prelude::*;
}

/// A module containing code related to Auto Tiles
///
/// Only accessible with the `auto-tile` feature enabled
#[cfg(feature = "auto-tile")]
pub mod auto {
	pub use bevy_tileset_core::auto::*;
}

/// Module containing items for debugging
pub mod debug {
	pub use bevy_tileset_core::debug::*;
}
