use serde::{Deserialize, Serialize};

/// An ID used to identify a [`Tileset`]
pub type TilesetId = u8;
/// An ID used to identify a tile in a [`Tileset`]
pub type TileGroupId = u32;
// /// An ID used to identify a variant in a variant tile
// #[cfg(feature = "variants")]
// pub type TileVariantId = u16;
// /// An ID used to identify a ruling in an auto tile
// #[cfg(feature = "auto-tile")]
// pub type TileAutoId = u16;

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
	pub group_id: TileGroupId,
	pub tileset_id: TilesetId,
}

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
	pub group_id: TileGroupId,
}

impl TileId {
	pub fn new(group_id: TileGroupId, tileset_id: TilesetId) -> Self {
		Self {
			#[cfg(feature = "auto-tile")]
			auto_index: None,
			#[cfg(feature = "variants")]
			variant_index: None,
			group_id,
			tileset_id,
		}
	}

	#[cfg(feature = "variants")]
	pub fn eq_variant(&self, other: &TileId) -> bool {
		self.variant_index == other.variant_index
			&& self.eq_tile_group(other)
			&& self.eq_tileset(other)
	}

	#[cfg(feature = "auto-tile")]
	pub fn eq_auto(&self, other: &TileId) -> bool {
		self.auto_index == other.auto_index && self.eq_tile_group(other) && self.eq_tileset(other)
	}

	pub fn eq_tile_group(&self, other: &TileId) -> bool {
		self.group_id == other.group_id && self.eq_tileset(other)
	}
	pub fn eq_tileset(&self, other: &TileId) -> bool {
		self.tileset_id == other.tileset_id
	}

	#[allow(dead_code)]
	pub(crate) fn partial(self) -> PartialTileId {
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
	pub fn new(group_id: TileGroupId) -> Self {
		Self {
			#[cfg(feature = "auto-tile")]
			auto_index: None,
			#[cfg(feature = "variants")]
			variant_index: None,
			group_id,
		}
	}

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

impl<T: Copy> From<&T> for PartialTileId
where
	PartialTileId: From<T>,
{
	fn from(item: &T) -> Self {
		Self::from(*item)
	}
}
