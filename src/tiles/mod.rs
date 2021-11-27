pub mod auto_tile;
pub mod data;
pub mod definitions;

pub use auto_tile::{AutoTile, AutoTileRule, RemoveAutoTileEvent};
pub use data::{
	AnimatedTileData, AutoTileData, SimpleTileType, TileData, TileType, VariantTileData,
};
pub use definitions::{
	AnimatedTileDef, AutoTileDef, SimpleTileDefType, TileDef, TileDefType, VariantTileDef,
};

pub(crate) mod internal {
	pub(crate) use super::data::{TryIntoTileData};
}
