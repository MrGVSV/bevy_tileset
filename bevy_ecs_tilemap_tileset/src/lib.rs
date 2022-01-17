pub use bevy_tileset as tileset;

#[cfg(feature = "auto-tile")]
pub(crate) mod auto;
mod placement;
mod plugin;

pub mod prelude {
	pub use bevy_tileset::prelude::*;

	#[cfg(feature = "auto-tile")]
	pub use super::auto::RemoveAutoTileEvent;
	pub use super::placement::*;
	pub use super::plugin::{TilesetLabel, TilesetPlugin, TilesetStage};
}
