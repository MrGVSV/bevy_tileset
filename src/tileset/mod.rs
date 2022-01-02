//! Tileset data and resources that contains, manages, and allows access to individual tiles

pub(crate) use asset::TilesetAssetLoader;
pub use asset::TilesetDef;
pub(crate) use param::TilesetMap;
pub use param::Tilesets;
pub use tile_index::TileIndex;
pub use tileset::{Tileset, TilesetBuilder};

mod asset;
pub mod error;
mod load;
mod param;
mod tile_index;
mod tileset;
