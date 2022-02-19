use serde::{Deserialize, Serialize};

/// An ID used to identify a [`Tileset`]
pub type TilesetId = u8;
/// An ID used to identify a tile in a [`Tileset`]
pub type TileGroupId = u32;

/// A struct used to identify a tile in a particular [`Tileset`]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct TileId {
	/// The specific index of a ruling in the list of rules for this auto tile
	///
	/// Only useful for Auto tiles
	#[cfg(feature = "auto-tile")]
	pub auto_index: Option<usize>,
	/// The specific index of a variant in the list of variants for this tile
	///
	/// Only useful for Variant tiles (and, by extension, Auto tiles)
	#[cfg(feature = "variants")]
	pub variant_index: Option<usize>,
	/// The tile group this tile belongs to
	pub group_id: TileGroupId,
	/// The ID of the containing [`Tileset`]
	pub tileset_id: TilesetId,
}

/// This struct is used to identify a tile when the particular [`Tileset`] is already known or unneeded
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PartialTileId {
	/// The specific index of a ruling in the list of rules for this auto tile
	///
	/// Only useful for Auto tiles
	#[cfg(feature = "auto-tile")]
	pub auto_index: Option<usize>,
	/// The specific index of a variant in the list of variants for this tile
	///
	/// Only useful for Variant tiles (and, by extension, Auto tiles)
	#[cfg(feature = "variants")]
	pub variant_index: Option<usize>,
	/// The tile group this tile belongs to
	pub group_id: TileGroupId,
}

impl TileId {
	/// Create a new basic tile ID
	pub const fn new(group_id: TileGroupId, tileset_id: TilesetId) -> Self {
		Self {
			#[cfg(feature = "auto-tile")]
			auto_index: None,
			#[cfg(feature = "variants")]
			variant_index: None,
			group_id,
			tileset_id,
		}
	}

	/// Returns true if two tiles are of the same variant, auto tile, group, and tileset
	#[cfg(feature = "variants")]
	pub fn eq_variant(&self, other: &TileId) -> bool {
		self.eq(other)
	}

	/// Returns true if two tiles are of the same auto tile, group, and tileset
	#[cfg(feature = "auto-tile")]
	pub fn eq_auto(&self, other: &TileId) -> bool {
		self.auto_index == other.auto_index && self.eq_tile_group(other)
	}

	/// Returns true if the two tiles are of the same group and tileset
	pub fn eq_tile_group(&self, other: &TileId) -> bool {
		self.group_id == other.group_id && self.eq_tileset(other)
	}

	/// Returns true if the two tiles are of the same tileset
	pub fn eq_tileset(&self, other: &TileId) -> bool {
		self.tileset_id == other.tileset_id
	}

	/// Creates a [`PartialTileId`] from this one
	pub fn partial(self) -> PartialTileId {
		PartialTileId {
			#[cfg(feature = "auto-tile")]
			auto_index: self.auto_index,
			#[cfg(feature = "variants")]
			variant_index: self.variant_index,
			group_id: self.group_id,
		}
	}
}

impl PartialTileId {
	pub const fn new(group_id: TileGroupId) -> Self {
		Self {
			#[cfg(feature = "auto-tile")]
			auto_index: None,
			#[cfg(feature = "variants")]
			variant_index: None,
			group_id,
		}
	}

	/// Extends this [`PartialTileId`] into a full [`TileId`]
	pub fn extend(self, tileset_id: TilesetId) -> TileId {
		TileId {
			#[cfg(feature = "auto-tile")]
			auto_index: self.auto_index,
			#[cfg(feature = "variants")]
			variant_index: self.variant_index,
			group_id: self.group_id,
			tileset_id,
		}
	}
}

impl From<TileId> for PartialTileId {
	fn from(id: TileId) -> Self {
		id.partial()
	}
}

impl From<TileGroupId> for PartialTileId {
	fn from(id: TileGroupId) -> Self {
		PartialTileId::new(id)
	}
}

impl<T: Copy + Into<PartialTileId>> From<&T> for PartialTileId {
	fn from(item: &T) -> Self {
		(*item).into()
	}
}
