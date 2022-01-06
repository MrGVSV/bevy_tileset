//! Tileset data and resources that contains, manages, and allows access to individual tiles

use std::collections::HashMap;

use bevy::prelude::{Handle, Texture, TextureAtlas, Vec2};
use bevy::reflect::TypeUuid;

pub(crate) use asset::TilesetAssetLoader;
pub use asset::TilesetDef;
pub use builder::TilesetBuilder;
pub use error::TilesetError;
pub use impls::*;
pub use load::load_tile_handles;
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
mod raw;
mod tile_index;

macro_rules! define_tileset {
	($(#[$attr:meta])* $vis: vis $name: ident { $($(#[$field_attr:meta])* $field: ident : $type: ty),* $(,)? }) => {
		$(#[$attr])*
		$vis struct $name {
			/// The ID of this tileset
			id: TilesetId,
			/// The name of this tileset
			name: String,
			/// The registered tiles mapped by their ID
			tiles: HashMap<TileGroupId, TileData>,
			/// The size of this tileset (in pixels)
			size: Vec2,
			/// The size of the tiles in this tileset (in pixels)
			tile_size: Vec2,
			/// The tile group IDs mapped by their name
			tile_ids: HashMap<String, TileGroupId>,
			/// The tile names mapped by their ID
			tile_names: HashMap<TileGroupId, String>,
			/// The tile handles mapped by their index in the atlas
			tile_handles: HashMap<usize, Handle<Texture>>,
			/// The tile IDs mapped by their index in the atlas
			tile_indices: HashMap<usize, TileId>,
			$(
				$(#[$field_attr])*
				$field : $type
			),*
		}
	};
}

define_tileset!(
	/// An intermediate structure containing the registered tiles as well as their generated `TextureAtlas`
	///
	/// This is useful for creating a tileset and using it immediately. Whereas, a standard [Tileset] breaks
	/// things up a bit more by transferring ownership of the `TextureAtlas` to the `Assets<TextureAtlas>` resource
	#[derive(Debug)]
	pub RawTileset {
		/// The atlas for all registered tiles
		atlas: TextureAtlas,
	}
);

define_tileset!(
	/// A structure containing the registered tiles as well as a handle to their generated `TextureAtlas`
	#[derive(Debug, TypeUuid)]
	#[uuid = "4a176882-d7b2-429d-af5c-be418ccc3c52"]
	pub Tileset {
		/// A handle to the generated texture atlas
		atlas: Handle<TextureAtlas>,
		/// A handle to the generated texture atlas's texture
		texture: Handle<Texture>
	}
);

/// A component used to pair a tile entity with the tileset it comes from
pub struct TilesetParent(pub TilesetId);
