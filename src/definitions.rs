//! Defines all necessary tile definitions for use in deserialization

use serde::{Deserialize, Serialize};

use crate::auto_tile::AutoTileRule;

/// Top-level tile definition structure
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TileDef {
	/// The name of this tile
	pub name: String,
	/// The actual tile data
	pub tile: TileDefType,
}

/// An enum defining the tile's type
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TileDefType {
	/// Defines a plain old tile
	Standard(String),
	/// Defines a tile with a frame-based animation
	Animated(AnimatedTileDef),
	/// Defines a set of tiles to randomly sample
	Variant(Vec<VariantTileDef>),
	/// Defines a set of tiles that chooses the one matching a given rule
	///
	/// > __Order here is important!__ Make sure tiles are listed in order of
	/// > descending rule restriction (i.e. the first item being the most restrictive)
	Auto(Vec<AutoTileDef>),
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

/// A structure defining an animated tile
///
/// Made to be easily used with [`bevy_ecs_tilemap::GPUAnimated`] component
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnimatedTileDef {
	/// The speed of the animation
	///
	/// Default: 1.0
	#[serde(default = "default_speed")]
	pub speed: f32,
	/// The frames of the animation
	///
	/// Each entry is a path to a texture relative to the configuration file
	///
	/// # Examples
	///
	/// ```ron
	/// (
	/// 	// ...
	/// 	frames: [
	/// 		"frame-001.png",
	/// 		"frame-002.png",
	/// 		"frame-003.png",
	/// 	]
	/// 	// ...
	/// )
	/// ```
	#[serde(default)]
	pub frames: Vec<String>,
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

/// A structure defining an auto tile
///
/// An auto tile contains rules that are applied when placed, removed, or changed
/// to itself and to its neighbors of the same type
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AutoTileDef {
	/// The rule defining this tile
	#[serde(default)]
	pub rule: AutoTileRule,
	/// The underlying tile variants
	#[serde(default)]
	pub variants: Vec<VariantTileDef>,
}

/// Gets the default animation speed
///
/// Used for deserialization
fn default_speed() -> f32 {
	1.0
}

/// Gets the default variant weight
///
/// Used for deserialization
fn default_weight() -> f32 {
	1.0
}
