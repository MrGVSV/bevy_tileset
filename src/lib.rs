mod auto_tile;
mod data;
pub mod debug;
mod definitions;
mod handles;
mod loader;
mod plugin;
mod tileset;

pub use auto_tile::{AutoTile, AutoTileRule, RemoveAutoTileEvent};
pub use data::{
	AnimatedTileData, AutoTileData, SimpleTileType, TileData, TileType, VariantTileData,
};
pub use definitions::{AnimatedTileDef, AutoTileDef, SimpleTileDefType, TileDef, VariantTileDef};
pub use loader::{TilesetDirs, TilesetLoadEvent, TilesetLoader};
pub use plugin::{TilesetLabel, TilesetPlugin, TilesetStage};
pub use tileset::{TileId, TileIndex, Tileset, TilesetBuilder, TilesetId, Tilesets};

pub mod load {
	pub use super::loader::{
		TilesetDirs, TilesetLoadEvent, TilesetLoader, DEFAULT_TILES_ASSET_DIR,
	};
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
