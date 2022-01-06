use bevy::prelude::{Handle, Texture, TextureAtlas, Vec2};

#[cfg(feature = "auto-tile")]
pub use auto::*;
#[cfg(feature = "variants")]
pub use variants::*;

use crate::prelude::*;

#[cfg(feature = "auto-tile")]
mod auto;
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

	/// Tries to get the [`TileIndex`] into the [`TextureAtlas`] for a tile with the given name
	///
	/// Auto tiles are given a default rule and will return indices for whatever matches first. To
	/// get the correct indices for tiles defined as [`TileType::Auto`], the [`get_auto_tile_index`]
	/// should be used instead.
	///
	/// However, keep in mind that the auto tile system should automatically pick up an auto tile,
	/// assuming it has the [`AutoTile`] component attached to it.
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	///
	/// returns: Option<TileIndex>
	///
	/// # Examples
	///
	/// ```
	/// # use bevy_tileset::prelude::*;
	///
	/// let index: TileIndex = tileset.get_tile_index("My Tile").unwrap();
	/// ```
	pub fn get_tile_index(&self, name: &str) -> Option<TileIndex> {
		let (index, ..) = self.select_tile(name)?;
		Some(index)
	}

	pub fn get_tile_index_by_id<TId: Into<PartialTileId>>(&self, id: TId) -> Option<TileIndex> {
		let (index, ..) = self.select_tile_by_id(id)?;
		Some(index)
	}

	/// Tries to get the base index into the [`TextureAtlas`] for a tile with the given name
	///
	/// This is a convenience method around [`get_tile_index`] that performs the match expression
	/// returning the index if [`TileIndex::Standard`] or the start index if [`TileIndex::Animated`]
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	///
	/// returns: Option<usize>
	///
	/// # Examples
	///
	/// ```
	/// let index: usize = tileset.get_base_tile_index("My Tile").unwrap();
	/// ```
	pub fn get_base_tile_index(&self, name: &str) -> Option<usize> {
		match self.get_tile_index(name)? {
			TileIndex::Standard(index) => Some(index),
			TileIndex::Animated(start, ..) => Some(start),
		}
	}

	/// Select a tile by its name
	///
	/// If the tile is a Variant tile, a random variant will be chosen.
	///
	/// If the tile is an Auto tile, the tile matching the default rule will be chosen.
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	///
	/// returns: Option<(TileIndex, &TileData)>
	///
	pub fn select_tile(&self, name: &str) -> Option<(TileIndex, &TileData)> {
		let group_id = self.get_tile_group_id(name)?;
		self.select_tile_by_id(group_id)
	}

	/// Select a tile by its ID
	///
	/// If the tile is a Variant tile, the designated variant will be chosen. Otherwise,
	/// a random variant will be chosen.
	///
	/// If the tile is an Auto tile, the designated auto tile will be chosen. Otherwise,
	/// the tile matching the default rule will be chosen.
	///
	/// # Arguments
	///
	/// * `tile_id`: The ID of the tile
	///
	/// returns: Option<(TileIndex, &TileData)>
	///
	pub fn select_tile_by_id<TId: Into<PartialTileId>>(
		&self,
		tile_id: TId,
	) -> Option<(TileIndex, &TileData)> {
		let id = tile_id.into();
		let group_id = id.group_id;
		let data = self.tiles.get(&group_id)?;

		Some((
			match data.tile() {
				TileType::Standard(index) => TileIndex::Standard(*index),
				TileType::Animated(anim) => {
					TileIndex::Animated(anim.start(), anim.end(), anim.speed())
				}
				#[cfg(feature = "variants")]
				TileType::Variant(variants) => {
					let variant = if let Some(idx) = id.variant_index {
						variants.get(idx)?
					} else {
						Self::select_variant(variants)?
					};
					variant.tile().into()
				}
				#[cfg(feature = "auto-tile")]
				TileType::Auto(autos) => Self::select_auto(autos, AutoTileRule::default(), id)?,
			},
			data,
		))
	}
}
