use bevy_ecs_tileset_tiles::prelude::*;

/// A structure defining the index or indexes into the texture atlas
#[derive(Debug, Copy, Clone)]
pub enum TileIndex {
	/// Index for a standard tile
	Standard(usize),
	/// Indexes for an animated tile.
	///
	/// Takes the form (start, end, speed)
	Animated(usize, usize, f32),
}

impl TileIndex {
	/// Get the base index
	///
	/// This is the regular index for [`TileIndex::Standard`] and the start index
	/// for [`TileIndex::Animated`]
	///
	pub fn base_index(&self) -> &usize {
		match self {
			Self::Standard(idx) => idx,
			Self::Animated(idx, ..) => idx,
		}
	}
}

impl From<AnimatedTileData> for TileIndex {
	fn from(data: AnimatedTileData) -> Self {
		TileIndex::Animated(data.start(), data.end(), data.speed())
	}
}

impl From<&AnimatedTileData> for TileIndex {
	fn from(data: &AnimatedTileData) -> Self {
		TileIndex::Animated(data.start(), data.end(), data.speed())
	}
}

#[cfg(feature = "variants")]
impl From<SimpleTileType> for TileIndex {
	fn from(data: SimpleTileType) -> Self {
		match data {
			SimpleTileType::Standard(index) => TileIndex::Standard(index),
			SimpleTileType::Animated(anim) => anim.into(),
		}
	}
}

#[cfg(feature = "variants")]
impl From<&SimpleTileType> for TileIndex {
	fn from(data: &SimpleTileType) -> Self {
		match data {
			SimpleTileType::Standard(index) => TileIndex::Standard(*index),
			SimpleTileType::Animated(anim) => anim.into(),
		}
	}
}
