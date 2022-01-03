//! Tileset data and resources that contains, manages, and allows access to individual tiles

use std::collections::HashMap;

use bevy::prelude::{Handle, Texture, TextureAtlas, Vec2};
use bevy::reflect::TypeUuid;

pub(crate) use asset::TilesetAssetLoader;
pub use asset::TilesetDef;
pub use builder::TilesetBuilder;
pub use error::TilesetError;
pub use impls::*;
pub(crate) use param::TilesetMap;
pub use param::Tilesets;
pub use tile_index::TileIndex;

use crate::prelude::*;

mod asset;
mod builder;
pub mod error;
mod impls;
mod load;
mod param;
mod tile_index;

/// A structure containing the registered tiles as well as their generated [`TextureAtlas`]
#[derive(Debug, TypeUuid)]
#[uuid = "4a176882-d7b2-429d-af5c-be418ccc3c52"]
pub struct Tileset {
	/// The ID of this tileset
	id: TilesetId,
	/// The name of this tileset
	name: String,
	/// The registered tiles mapped by their ID
	tiles: HashMap<TileGroupId, TileData>,
	/// The atlas for all registered tiles
	atlas: TextureAtlas,
	/// The size of the tiles in this tileset
	tile_size: Vec2,
	/// The tile group IDs mapped by their name
	tile_ids: HashMap<String, TileGroupId>,
	/// The tile names mapped by their ID
	tile_names: HashMap<TileGroupId, String>,
	/// The tile handles mapped by their index in the atlas
	tile_handles: HashMap<usize, Handle<Texture>>,
	/// The tile IDs mapped by their index in the atlas
	tile_indices: HashMap<usize, TileId>,
}
