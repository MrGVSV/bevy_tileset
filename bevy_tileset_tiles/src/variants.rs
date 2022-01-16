use crate::prelude::{AnimatedTileData, AnimatedTileDef, AnimatedTileHandle};
use bevy_asset::Handle;
use bevy_render::texture::Image;
use serde::{Deserialize, Serialize};

/// A structure defining a _variant_ tile
///
/// A _variant_ essentially wraps a [simple](SimpleTileType) tile and gives it
/// a weight. This weight is used to define how likely it should be picked at random
#[derive(Debug, Copy, Clone, Serialize)]
pub struct VariantTileData {
	/// The weight of this variant (used for random sampling)
	weight: f32,
	/// The underlying tile
	tile: SimpleTileType,
}

/// An enum defining "simple" tile types
///
/// These are "simple" types in that their inner types are not _too_ complex
/// or heavily nested
#[derive(Debug, Copy, Clone, Serialize)]
pub enum SimpleTileType {
	Standard(usize),
	Animated(AnimatedTileData),
}

/// A structure defining a _variant_ tile
#[derive(Debug, Clone)]
pub struct VariantTileHandle {
	/// The weight of this variant (used for random sampling)
	pub weight: f32,
	/// The underlying tile handle
	pub tile: SimpleTileHandle,
}

/// An enum defining "simple" tile types
#[derive(Debug, Clone)]
pub enum SimpleTileHandle {
	Standard(Handle<Image>),
	Animated(AnimatedTileHandle),
}

/// A structure defining a _variant_ tile
///
/// A _variant_ essentially wraps a [simple](SimpleTileDefType) tile and gives it
/// a weight. This weight is used to define how likely it should be picked at random
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VariantTileDef {
	/// The weight of this variant (used for random sampling)
	///
	/// Default: 1.0
	#[serde(default = "default_weight")]
	pub weight: f32,
	/// The underlying tile
	pub tile: SimpleTileDefType,
}

/// An enum defining "simple" tile types
///
/// These are "simple" types in that their inner types are not _too_ complex
/// or heavily nested
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SimpleTileDefType {
	Standard(String),
	Animated(AnimatedTileDef),
}

impl VariantTileData {
	pub fn new(weight: f32, tile: SimpleTileType) -> Self {
		Self { weight, tile }
	}

	/// Gets the weight of this variant
	pub fn weight(&self) -> f32 {
		self.weight
	}

	/// Gets the underlying tile data
	pub fn tile(&self) -> &SimpleTileType {
		&self.tile
	}
}

impl SimpleTileType {
	/// Checks if the given index exists within this tile
	///
	/// # Arguments
	///
	/// * `index`: The index to check
	///
	/// returns: bool
	///
	pub fn contains_index(&self, index: &usize) -> bool {
		match self {
			Self::Standard(idx) => idx == index,
			Self::Animated(anim) => anim.start() <= *index && *index <= anim.end(),
		}
	}
}

/// Gets the default variant weight
///
/// Used for deserialization
fn default_weight() -> f32 {
	1.0
}
