//! Tile data, including tile definitions (for config files) and auto tiling
pub mod prelude {
	pub use super::animated::{AnimatedTileData, AnimatedTileDef, AnimatedTileHandle};
	#[cfg(feature = "auto-tile")]
	pub use super::auto::{AutoTileData, AutoTileDef, AutoTileHandle, AutoTileRule};
	pub use super::tile::{TileData, TileDef, TileDefType, TileHandle, TileHandleType, TileType};
	#[cfg(feature = "variants")]
	pub use super::variants::{
		SimpleTileDefType, SimpleTileHandle, SimpleTileType, VariantTileData, VariantTileDef,
		VariantTileHandle,
	};
}

pub mod animated;
#[cfg(feature = "auto-tile")]
pub mod auto;
pub mod tile;
#[cfg(feature = "variants")]
pub mod variants;
