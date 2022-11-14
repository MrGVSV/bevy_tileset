use bevy_asset::{AssetServer, Handle, LoadState};
use bevy_render::texture::Image;
use serde::{Deserialize, Serialize};

#[cfg(feature = "auto-tile")]
use crate::auto::*;
use crate::prelude::{AnimatedTileData, AnimatedTileDef, AnimatedTileHandle};
#[cfg(feature = "variants")]
use crate::variants::*;

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
	#[cfg(feature = "variants")]
	Variant(Vec<VariantTileData>),
	/// A collection of auto tiles
	#[cfg(feature = "auto-tile")]
	Auto(Vec<AutoTileData>),
}

/// Top-level structure defining a tile
#[derive(Debug, Clone)]
pub struct TileHandle {
	pub name: String,
	pub tile: TileHandleType,
}

/// An enum defining the tile's type
#[derive(Debug, Clone)]
pub enum TileHandleType {
	Standard(Handle<Image>),
	Animated(AnimatedTileHandle),
	#[cfg(feature = "variants")]
	Variant(Vec<VariantTileHandle>),
	#[cfg(feature = "auto-tile")]
	Auto(Vec<AutoTileHandle>),
}

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
	#[cfg(feature = "variants")]
	Variant(Vec<VariantTileDef>),
	/// Defines a set of tiles that chooses the one matching a given rule
	///
	/// > __Order here is important!__ Make sure tiles are listed in order of
	/// > descending rule restriction (i.e. the first item being the most restrictive)
	#[cfg(feature = "auto-tile")]
	Auto(Vec<AutoTileDef>),
}

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
		matches!(self.tile, TileType::Standard(..))
	}

	/// Checks if the underlying tile is a [`TileType::Animated`] tile
	pub fn is_animated(&self) -> bool {
		matches!(self.tile, TileType::Animated(..))
	}

	/// Checks if the underlying tile is a [`TileType::Variant`] tile
	#[cfg(feature = "variants")]
	pub fn is_variant(&self) -> bool {
		matches!(self.tile, TileType::Variant(..))
	}

	/// Checks if the underlying tile is a [`TileType::Auto`] tile
	#[cfg(feature = "auto-tile")]
	pub fn is_auto(&self) -> bool {
		matches!(self.tile, TileType::Auto(..))
	}
}

impl TileType {
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
			#[cfg(feature = "variants")]
			Self::Variant(variants) => variants.iter().any(|v| v.tile().contains_index(index)),
			#[cfg(feature = "auto-tile")]
			Self::Auto(autos) => autos
				.iter()
				.flat_map(|a| a.variants())
				.any(|v| v.tile().contains_index(index)),
		}
	}
}

impl TileHandle {
	pub fn new_standard<TName: Into<String>>(name: TName, handle: Handle<Image>) -> Self {
		Self {
			name: name.into(),
			tile: TileHandleType::Standard(handle),
		}
	}

	pub fn new_animated<TName: Into<String>>(name: TName, handle: AnimatedTileHandle) -> Self {
		Self {
			name: name.into(),
			tile: TileHandleType::Animated(handle),
		}
	}

	#[cfg(feature = "variants")]
	pub fn new_variant<TName: Into<String>>(name: TName, handles: Vec<VariantTileHandle>) -> Self {
		Self {
			name: name.into(),
			tile: TileHandleType::Variant(handles.clone()),
		}
	}

	#[cfg(feature = "auto-tile")]
	pub fn new_auto<TName: Into<String>>(name: TName, handles: Vec<AutoTileHandle>) -> Self {
		Self {
			name: name.into(),
			tile: TileHandleType::Auto(handles.clone()),
		}
	}

	pub fn is_loaded(&self, asset_server: &AssetServer) -> bool {
		self.get_load_state(asset_server) == LoadState::Loaded
	}

	pub fn get_load_state(&self, asset_server: &AssetServer) -> LoadState {
		asset_server.get_group_load_state(self.iter_handles().map(|handle| handle.id()))
	}

	pub fn iter_handles(&self) -> Box<dyn Iterator<Item = &Handle<Image>> + '_> {
		match &self.tile {
			TileHandleType::Standard(handle) => Box::new(std::iter::once(handle)),
			TileHandleType::Animated(anim) => Box::new(anim.frames.iter()),
			#[cfg(feature = "variants")]
			TileHandleType::Variant(variants) => Box::new(iter_variant_handles(variants.iter())),
			#[cfg(feature = "auto-tile")]
			TileHandleType::Auto(autos) => Box::new(iter_variant_handles(
				autos.iter().flat_map(|auto| auto.variants.iter()),
			)),
		}
	}
}

#[cfg(feature = "variants")]
fn iter_variant_handles<'a>(
	variants: impl Iterator<Item = &'a VariantTileHandle>,
) -> impl Iterator<Item = &'a Handle<Image>> {
	variants
		.map(|variant| {
			let iter: Box<dyn Iterator<Item = &Handle<Image>>> = match &variant.tile {
				SimpleTileHandle::Standard(handle) => Box::new(std::iter::once(handle)),
				SimpleTileHandle::Animated(anim) => Box::new(anim.frames.iter()),
			};
			iter
		})
		.flat_map(|x| x)
}

#[cfg(test)]
mod tests {
	use bevy_asset::Handle;

	use crate::prelude::*;

	#[test]
	fn should_iter_standard() {
		let standard = TileHandle::new_standard("Standard", Handle::default());
		let mut standard_iter = standard.iter_handles();
		// Standard (1)
		assert!(standard_iter.next().is_some());
		// End
		assert!(standard_iter.next().is_none());
	}

	#[test]
	fn should_iter_animated() {
		let anim = TileHandle::new_animated(
			"Animated",
			AnimatedTileHandle {
				speed: 1.0,
				frames: vec![Handle::default(); 3],
			},
		);
		let mut anim_iter = anim.iter_handles();
		// Animated (3)
		assert!(anim_iter.next().is_some());
		assert!(anim_iter.next().is_some());
		assert!(anim_iter.next().is_some());
		// End
		assert!(anim_iter.next().is_none());
	}

	#[cfg(feature = "variants")]
	#[test]
	fn should_iter_variant() {
		let variant = TileHandle::new_variant(
			"Variant",
			vec![
				VariantTileHandle {
					weight: 1.0,
					tile: SimpleTileHandle::Standard(Handle::default()),
				},
				VariantTileHandle {
					weight: 1.0,
					tile: SimpleTileHandle::Animated(AnimatedTileHandle {
						speed: 1.0,
						frames: vec![Handle::default(); 3],
					}),
				},
			],
		);
		let mut variant_iter = variant.iter_handles();
		// Variant #1 - Standard (1)
		assert!(variant_iter.next().is_some());
		// Variant #2 - Animated (3)
		assert!(variant_iter.next().is_some());
		assert!(variant_iter.next().is_some());
		assert!(variant_iter.next().is_some());
		// End
		assert!(variant_iter.next().is_none());
	}

	#[cfg(feature = "auto-tile")]
	#[test]
	fn should_iter_auto() {
		let auto = TileHandle::new_auto(
			"Auto",
			vec![
				AutoTileHandle {
					rule: AutoTileRule::default(),
					variants: vec![
						VariantTileHandle {
							weight: 1.0,
							tile: SimpleTileHandle::Standard(Handle::default()),
						},
						VariantTileHandle {
							weight: 1.0,
							tile: SimpleTileHandle::Animated(AnimatedTileHandle {
								speed: 1.0,
								frames: vec![Handle::default(); 3],
							}),
						},
					],
				},
				AutoTileHandle {
					rule: AutoTileRule::default(),
					variants: vec![
						VariantTileHandle {
							weight: 1.0,
							tile: SimpleTileHandle::Standard(Handle::default()),
						},
						VariantTileHandle {
							weight: 1.0,
							tile: SimpleTileHandle::Animated(AnimatedTileHandle {
								speed: 1.0,
								frames: vec![Handle::default(); 3],
							}),
						},
					],
				},
			],
		);

		let mut auto_iter = auto.iter_handles();
		// Rule #1
		// Variant #1 - Standard (1)
		assert!(auto_iter.next().is_some());
		// Variant #2 - Animated (3)
		assert!(auto_iter.next().is_some());
		assert!(auto_iter.next().is_some());
		assert!(auto_iter.next().is_some());
		// Rule #2
		// Variant #1 - Standard (1)
		assert!(auto_iter.next().is_some());
		// Variant #2 - Animated (3)
		assert!(auto_iter.next().is_some());
		assert!(auto_iter.next().is_some());
		assert!(auto_iter.next().is_some());
		// End
		assert!(auto_iter.next().is_none());
	}
}
