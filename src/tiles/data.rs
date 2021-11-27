//! Module defining various generated tile data structures
//!
//! These structures should only ever be generated alongside their respective
//! [`Tileset`](tileset::Tileset) since they mostly define their indices into the [`TextureAtlas`]

use crate::handles::{
	AnimatedTileHandle, AutoTileHandle, SimpleTileHandle, TileHandle, TileHandleBase,
	VariantTileHandle,
};
use crate::tiles::auto_tile::AutoTileRule;
use crate::tileset::{TileIndex, TilesetBuilder};
use bevy::prelude::{Assets, Handle, Texture};
use serde::Serialize;

/// Top-level structure defining a tile
#[derive(Debug, Clone, Serialize)]
pub struct TileData {
	/// The name of this tile
	name: String,
	/// The actual tile data
	tile: TileType,
}

/// An enum defining the tile's type
#[derive(Debug, Clone, Serialize)]
pub enum TileType {
	/// A standard tile
	Standard(usize),
	/// A frame-based animated tile
	Animated(AnimatedTileData),
	/// A collection of tiles to randomly sample
	Variant(Vec<VariantTileData>),
	/// A collection of auto tiles
	Auto(Vec<AutoTileData>),
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

/// A structure defining an animated tile
///
/// Made to be easily used with [`bevy_ecs_tilemap::GPUAnimated`] component
#[derive(Debug, Copy, Clone, Serialize)]
pub struct AnimatedTileData {
	/// The speed of the animation
	speed: f32,
	/// The start index of the animation (inclusive)
	start: usize,
	/// The end index of the animation (inclusive)
	end: usize,
}

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

/// A structure defining an auto tile
///
/// An auto tile contains rules that are applied when placed, removed, or changed
/// to itself and to its neighbors of the same type
#[derive(Debug, Clone, Serialize)]
pub struct AutoTileData {
	/// The rule defining this tile
	rule: AutoTileRule,
	/// The underlying tile variants
	variants: Vec<VariantTileData>,
}

pub trait TileDataTrait {}

macro_rules! impl_tile_data {
	($name: ty) => {
		impl TileDataTrait for $name {}
	};
}

//    _____                 _
//   |_   _|               | |
//     | |  _ __ ___  _ __ | |___
//     | | | '_ ` _ \| '_ \| / __|
//    _| |_| | | | | | |_) | \__ \
//   |_____|_| |_| |_| .__/|_|___/
//                   | |
//                   |_|

impl_tile_data!(usize);
impl_tile_data!(SimpleTileType);
impl_tile_data!(AnimatedTileData);
impl_tile_data!(VariantTileData);
impl_tile_data!(Vec<VariantTileData>);
impl_tile_data!(AutoTileData);
impl_tile_data!(Vec<AutoTileData>);

impl TileData {
	/// Create a new [`TileData`] instance
	///
	/// # Arguments
	///
	/// * `name`: The name of this tile
	/// * `tile`: The underlying tile data
	///
	/// returns: TileData
	///
	/// # Examples
	///
	/// ```
	/// let some_texture_index = 0usize;
	///
	/// let tile = TileData::new(
	/// 	String::from("My Tile"),
	/// 	TileType::Standard(some_texture_index)
	/// );
	/// ```
	pub fn new(name: String, tile: TileType) -> Self {
		Self { name, tile }
	}

	/// Gets the name of this tile
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Gets the underlying tile data
	pub fn tile(&self) -> &TileType {
		&self.tile
	}

	/// Checks if the underlying tile is a [`TileType::Standard`] tile
	pub fn is_standard(&self) -> bool {
		match self.tile {
			TileType::Standard(_) => true,
			_ => false,
		}
	}

	/// Checks if the underlying tile is a [`TileType::Animated`] tile
	pub fn is_animated(&self) -> bool {
		match self.tile {
			TileType::Animated(_) => true,
			_ => false,
		}
	}

	/// Checks if the underlying tile is a [`TileType::Variant`] tile
	pub fn is_variant(&self) -> bool {
		match self.tile {
			TileType::Variant(_) => true,
			_ => false,
		}
	}

	/// Checks if the underlying tile is a [`TileType::Auto`] tile
	pub fn is_auto(&self) -> bool {
		match self.tile {
			TileType::Auto(_) => true,
			_ => false,
		}
	}
}

impl AnimatedTileData {
	/// Gets the start animation index (inclusive)
	pub fn start(&self) -> usize {
		self.start
	}

	/// Gets the end animation index (inclusive)
	pub fn end(&self) -> usize {
		self.end
	}

	/// Gets the animation speed
	pub fn speed(&self) -> f32 {
		self.speed
	}

	/// Gets the number of frames in this animation
	pub fn frame_count(&self) -> usize {
		self.end - self.start
	}
}

impl VariantTileData {
	/// Gets the weight of this variant
	pub fn weight(&self) -> f32 {
		self.weight
	}

	/// Gets the underlying tile data
	pub fn tile(&self) -> &SimpleTileType {
		&self.tile
	}
}

impl AutoTileData {
	/// Gets the rule associated with this auto tile
	pub fn rule(&self) -> AutoTileRule {
		self.rule
	}

	/// Gets the underlying tile variants
	pub fn variants(&self) -> &Vec<VariantTileData> {
		&self.variants
	}
}

impl TileType {
	/// Attempts to convert a [tile handle](TileHandleBase) to a valid [`TileType`]
	///
	/// This should almost always contain [`Some`] value. If [`None`] is returned, then
	/// it likely has to do with the textures not being fully/properly loaded before calling
	/// this method.
	///
	/// # Arguments
	///
	/// * `tile`: The tile handle object
	/// * `atlas`: The atlas builder
	/// * `texture_store`: The texture assets
	///
	/// returns: Option<TileType>
	pub(crate) fn add_to_tileset(
		tile: TileHandleBase,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self> {
		Some(match tile.tile {
			TileHandle::Standard(handle) => {
				let index = handle.try_into_tile_data(builder, texture_store)?;
				TileType::Standard(index)
			}
			TileHandle::Animated(anim) => {
				let anim = anim.try_into_tile_data(builder, texture_store)?;
				TileType::Animated(anim)
			}
			TileHandle::Variant(variants) => {
				let variants = variants.try_into_tile_data(builder, texture_store)?;
				TileType::Variant(variants)
			}
			TileHandle::Auto(autos) => {
				let autos = autos.try_into_tile_data(builder, texture_store)?;
				TileType::Auto(autos)
			}
		})
	}

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
			Self::Variant(variants) => variants.iter().any(|v| v.tile().contains_index(index)),
			Self::Auto(autos) => autos
				.iter()
				.flat_map(|a| a.variants())
				.any(|v| v.tile().contains_index(index)),
		}
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

//    ___       _          _____ _ _        ___           _
//   |_ _|_ __ | |_ ___   |_   _(_) | ___  |_ _|_ __   __| | _____  __
//    | || '_ \| __/ _ \    | | | | |/ _ \  | || '_ \ / _` |/ _ \ \/ /
//    | || | | | || (_) |   | | | | |  __/  | || | | | (_| |  __/>  <
//   |___|_| |_|\__\___/    |_| |_|_|\___| |___|_| |_|\__,_|\___/_/\_\
//

impl Into<TileIndex> for &AnimatedTileData {
	fn into(self) -> TileIndex {
		TileIndex::Animated(self.start(), self.end(), self.speed())
	}
}

impl Into<TileIndex> for &SimpleTileType {
	fn into(self) -> TileIndex {
		match self {
			SimpleTileType::Standard(index) => TileIndex::Standard(*index),
			SimpleTileType::Animated(anim) => anim.into(),
		}
	}
}

//    ___       _          _____ _ _        ____        _
//   |_ _|_ __ | |_ ___   |_   _(_) | ___  |  _ \  __ _| |_ __ _
//    | || '_ \| __/ _ \    | | | | |/ _ \ | | | |/ _` | __/ _` |
//    | || | | | || (_) |   | | | | |  __/ | |_| | (_| | || (_| |
//   |___|_| |_|\__\___/    |_| |_|_|\___| |____/ \__,_|\__\__,_|
//

pub(crate) trait TryIntoTileData {
	type DataType: TileDataTrait;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType>;
}

impl TryIntoTileData for Handle<Texture> {
	type DataType = usize;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType> {
		builder.insert_handle(&self, texture_store)
	}
}

impl TryIntoTileData for AnimatedTileHandle {
	type DataType = AnimatedTileData;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType> {
		let (mut start, mut end) = (-1, -1);
		for frame in self.frames {
			let index = builder.insert_handle(&frame, texture_store)?;
			if start == -1 {
				start = index as i32;
			} else {
				end = index as i32;
			}
		}

		if start < 0 || end < 0 {
			// Invalid indexes
			return None;
		}

		Some(AnimatedTileData {
			speed: self.speed,
			start: start as usize,
			end: end as usize,
		})
	}
}

impl TryIntoTileData for VariantTileHandle {
	type DataType = VariantTileData;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType> {
		Some(VariantTileData {
			weight: self.weight,
			tile: match self.tile {
				SimpleTileHandle::Standard(handle) => {
					let index = handle.try_into_tile_data(builder, texture_store)?;
					SimpleTileType::Standard(index)
				}
				SimpleTileHandle::Animated(anim) => {
					let anim_data = anim.try_into_tile_data(builder, texture_store)?;
					SimpleTileType::Animated(anim_data)
				}
			},
		})
	}
}

impl TryIntoTileData for Vec<VariantTileHandle> {
	type DataType = Vec<VariantTileData>;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType> {
		let mut data: Vec<VariantTileData> = Vec::with_capacity(self.len());
		for variant in self {
			let variant = variant.try_into_tile_data(builder, texture_store)?;
			data.push(variant);
		}
		Some(data)
	}
}

impl TryIntoTileData for AutoTileHandle {
	type DataType = AutoTileData;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType> {
		Some(AutoTileData {
			rule: self.rule,
			variants: self.variants.try_into_tile_data(builder, texture_store)?,
		})
	}
}

impl TryIntoTileData for Vec<AutoTileHandle> {
	type DataType = Vec<AutoTileData>;

	fn try_into_tile_data(
		self,
		builder: &mut TilesetBuilder,
		texture_store: &Assets<Texture>,
	) -> Option<Self::DataType> {
		let mut data: Vec<AutoTileData> = Vec::with_capacity(self.len());
		for auto in self {
			let auto = auto.try_into_tile_data(builder, texture_store)?;
			data.push(auto);
		}
		Some(data)
	}
}
