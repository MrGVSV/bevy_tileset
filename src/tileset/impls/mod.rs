use bevy::prelude::{Handle, Texture, TextureAtlas, Vec2};

#[cfg(feature = "auto-tile")]
pub use auto::*;
pub use placement::*;
#[cfg(feature = "variants")]
pub use variants::*;

use crate::prelude::*;

#[cfg(feature = "auto-tile")]
mod auto;
mod placement;
#[cfg(feature = "variants")]
mod variants;

impl Tileset {
	/// Gets the name of this tileset
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Gets the ID of this tileset
	pub fn id(&self) -> &TilesetId {
		&self.id
	}

	/// Gets the tileset [`TextureAtlas`]
	pub fn atlas(&self) -> &TextureAtlas {
		&self.atlas
	}

	/// Gets the handle to the [`TextureAtlas`]'s texture
	pub fn texture(&self) -> &Handle<Texture> {
		&self.atlas.texture
	}

	/// Gets the tile size for this tileset
	pub fn tile_size(&self) -> Vec2 {
		self.tile_size
	}

	/// Get the name of a tile by its group ID
	///
	/// # Arguments
	///
	/// * `id`: The tile's ID
	///
	/// returns: Option<&String>
	///
	pub fn get_tile_name(&self, group_id: &TileGroupId) -> Option<&String> {
		self.tile_names.get(&group_id)
	}

	/// Get the base tile name for the given index
	///
	/// # Arguments
	///
	/// * `index`: The texture index
	///
	/// returns: Option<&String>
	///
	pub fn get_tile_name_by_index(&self, index: &usize) -> Option<&String> {
		let TileId { group_id, .. } = self.tile_indices.get(index)?;
		self.get_tile_name(group_id)
	}

	/// Get the group ID of a tile by its name
	///
	/// # Arguments
	///
	/// * `name`: The tile's name
	///
	/// returns: Option<&u32>
	///
	pub fn get_tile_group_id(&self, name: &str) -> Option<&TileGroupId> {
		self.tile_ids.get(name)
	}

	/// Get the ID of a tile by its index in the texture atlas
	///
	/// # Arguments
	///
	/// * `index`: The tile's index
	///
	/// returns: Option<&u32>
	///
	pub fn get_tile_id(&self, index: &usize) -> Option<&TileId> {
		self.tile_indices.get(index)
	}

	/// Get the handle of a tile by its index in the texture atlas
	///
	/// # Arguments
	///
	/// * `index`: The tile's index
	///
	/// returns: Option<&Handle<Texture>>
	///
	pub fn get_tile_handle(&self, index: &usize) -> Option<&Handle<Texture>> {
		self.tile_handles.get(index)
	}

	/// Get the data of a tile by its name
	///
	/// # Arguments
	///
	/// * `name`: The tile's name
	///
	/// returns: Option<&TileData>
	///
	pub fn get_tile_data(&self, name: &str) -> Option<&TileData> {
		let id = self.tile_ids.get(name)?;
		self.tiles.get(id)
	}
}
