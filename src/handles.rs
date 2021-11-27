//! Contains intermediary types for tile data. Specifically, these types hold
//! onto necessary information for generating tiles, including handles to their
//! respective textures

use std::path::{Path, PathBuf};

use bevy::asset::LoadState;
use bevy::prelude::{AssetServer, Handle, Texture};


use crate::tiles::auto_tile::AutoTileRule;
use crate::tiles::definitions::{
	AnimatedTileDef, AutoTileDef, SimpleTileDefType, TileDef, TileDefType, VariantTileDef,
};


/// Resource containing all tile handles waiting to be processed
#[derive(Debug, Default, Clone)]
pub(crate) struct TilesetHandles {
	pub(crate) is_dirty: bool,
	pub(crate) tiles: Vec<TileHandleBase>,
}

/// Top-level structure defining a tile
#[derive(Debug, Clone)]
pub(crate) struct TileHandleBase {
	pub(crate) name: String,
	pub(crate) tile: TileHandle,
}

/// An enum defining the tile's type
#[derive(Debug, Clone)]
pub(crate) enum TileHandle {
	Standard(Handle<Texture>),
	Animated(AnimatedTileHandle),
	Variant(Vec<VariantTileHandle>),
	Auto(Vec<AutoTileHandle>),
}

/// An enum defining "simple" tile types
#[derive(Debug, Clone)]
pub(crate) enum SimpleTileHandle {
	Standard(Handle<Texture>),
	Animated(AnimatedTileHandle),
}

/// A structure defining an animated tile
#[derive(Debug, Clone)]
pub(crate) struct AnimatedTileHandle {
	/// The speed of the animation
	pub(crate) speed: f32,
	/// The frames of the animation
	///
	/// Each frame is a registered [`Handle`]
	pub(crate) frames: Vec<Handle<Texture>>,
}

/// A structure defining a _variant_ tile
#[derive(Debug, Clone)]
pub(crate) struct VariantTileHandle {
	/// The weight of this variant (used for random sampling)
	pub(crate) weight: f32,
	/// The underlying tile handle
	pub(crate) tile: SimpleTileHandle,
}

/// A structure defining an auto tile
#[derive(Debug, Clone)]
pub(crate) struct AutoTileHandle {
	/// The rule defining this tile
	pub(crate) rule: AutoTileRule,
	/// The underlying variant handles
	pub(crate) variants: Vec<VariantTileHandle>,
}

pub trait TileHandleTrait {}

macro_rules! impl_tile_handle {
	($name: ty) => {
		impl TileHandleTrait for $name {}
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

impl_tile_handle!(Handle<Texture>);
impl_tile_handle!(SimpleTileHandle);
impl_tile_handle!(AnimatedTileHandle);
impl_tile_handle!(VariantTileHandle);
impl_tile_handle!(Vec<VariantTileHandle>);
impl_tile_handle!(AutoTileHandle);
impl_tile_handle!(Vec<AutoTileHandle>);
impl_tile_handle!(TileHandle);

impl TileHandleBase {
	/// Checks if all textures for this tile are loaded
	///
	/// # Arguments
	///
	/// * `asset_server`: The world's [`AssetServer`]
	///
	/// returns: bool
	///
	pub(crate) fn is_loaded(&self, asset_server: &AssetServer) -> bool {
		let ids = self.tile.iter_handle().map(|handle| handle.id);
		LoadState::Loaded == asset_server.get_group_load_state(ids)
	}
}

impl TilesetHandles {
	/// Checks if all textures for all tiles are loaded
	///
	/// # Arguments
	///
	/// * `asset_server`: The world's [`AssetServer`]
	///
	/// returns: bool
	///
	pub(crate) fn is_loaded(&self, asset_server: &AssetServer) -> bool {
		self.tiles.iter().all(|tile| tile.is_loaded(asset_server))
	}

	/// Add a [`TileHandle`] to be loaded
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	/// * `handle`: The tile handle
	///
	/// returns: ()
	///
	pub(crate) fn add_handle(&mut self, name: String, handle: TileHandle) {
		self.tiles.push(TileHandleBase { name, tile: handle })
	}

	/// Load a [`TileDef`]
	///
	/// # Arguments
	///
	/// * `tile`: The tile definition to load
	/// * `texture_dir`: The asset directory to load textures from (i.e. "images" for "project/assets/images")
	/// * `asset_server`: The world asset server
	///
	/// returns: ()
	///
	/// # Examples
	///
	/// ```
	/// use bevy::prelude::*;
	/// use bevy_ecs_tilemap_tileset::TileDef;
	///
	/// fn load(mut handles: ResMut<TilesetHandles>, asset_server: Res<AssetServer>) {
	/// 	let image_dir = String::from("images");
	///    	let tile_def = TileDef {
	/// 		name: String::from("My Tile"),
	/// 		tile: TileDefType::Standard(String::from("my_tile.png"))
	/// 	};
	/// 	handles.add_tile(tile_def, &image_dir, &asset_server);
	/// }
	/// ```
	pub(crate) fn add_tile(
		&mut self,
		tile: TileDef,
		texture_dir: &str,
		asset_server: &AssetServer,
	) {
		self.add_handle(tile.name, tile.tile.as_handle(texture_dir, asset_server));
	}

	pub(crate) fn len(&self) -> usize {
		self.tiles.len()
	}
}

//       _          _   _                 _ _
//      / \   ___  | | | | __ _ _ __   __| | | ___
//     / _ \ / __| | |_| |/ _` | '_ \ / _` | |/ _ \
//    / ___ \\__ \ |  _  | (_| | | | | (_| | |  __/
//   /_/   \_\___/ |_| |_|\__,_|_| |_|\__,_|_|\___|
//

pub(crate) trait AsTileHandle {
	type HandleType;
	fn as_handle(&self, root: &str, asset_server: &AssetServer) -> Self::HandleType;

	fn make_path<'a>(root: &'a str, path: &'a str) -> PathBuf {
		Path::new(root).join(path)
	}
}

impl AsTileHandle for TileDefType {
	type HandleType = TileHandle;

	fn as_handle(&self, root: &str, asset_server: &AssetServer) -> Self::HandleType {
		match self {
			Self::Standard(path) => {
				TileHandle::Standard(asset_server.load(Self::make_path(root, path)))
			}
			Self::Animated(anim) => TileHandle::Animated(anim.as_handle(root, asset_server)),
			Self::Variant(variants) => TileHandle::Variant(
				variants
					.iter()
					.map(|variant| variant.as_handle(root, asset_server))
					.collect(),
			),
			Self::Auto(autos) => TileHandle::Auto(
				autos
					.iter()
					.map(|auto| auto.as_handle(root, asset_server))
					.collect(),
			),
		}
	}
}

impl AsTileHandle for AnimatedTileDef {
	type HandleType = AnimatedTileHandle;

	fn as_handle(&self, root: &str, asset_server: &AssetServer) -> Self::HandleType {
		AnimatedTileHandle {
			speed: self.speed,
			frames: self
				.frames
				.iter()
				.map(|frame| asset_server.load(Self::make_path(root, frame)))
				.collect(),
		}
	}
}

impl AsTileHandle for VariantTileDef {
	type HandleType = VariantTileHandle;

	fn as_handle(&self, root: &str, asset_server: &AssetServer) -> Self::HandleType {
		VariantTileHandle {
			tile: match self.tile {
				SimpleTileDefType::Standard(ref path) => {
					SimpleTileHandle::Standard(asset_server.load(Self::make_path(root, path)))
				}
				SimpleTileDefType::Animated(ref anim) => {
					SimpleTileHandle::Animated(anim.as_handle(root, asset_server))
				}
			},
			weight: self.weight,
		}
	}
}

impl AsTileHandle for AutoTileDef {
	type HandleType = AutoTileHandle;

	fn as_handle(&self, root: &str, asset_server: &AssetServer) -> Self::HandleType {
		AutoTileHandle {
			rule: self.rule,
			variants: self
				.variants
				.iter()
				.map(|variant| variant.as_handle(root, asset_server))
				.collect(),
		}
	}
}

//    ___ _              _   _                 _ _
//   |_ _| |_ ___ _ __  | | | | __ _ _ __   __| | | ___
//    | || __/ _ \ '__| | |_| |/ _` | '_ \ / _` | |/ _ \
//    | || ||  __/ |    |  _  | (_| | | | | (_| | |  __/
//   |___|\__\___|_|    |_| |_|\__,_|_| |_|\__,_|_|\___|
//

trait IterHandle {
	fn iter_handle(&self) -> Box<dyn Iterator<Item = &Handle<Texture>> + '_>;
}

impl IterHandle for AnimatedTileHandle {
	fn iter_handle(&self) -> Box<dyn Iterator<Item = &Handle<Texture>> + '_> {
		Box::new(self.frames.iter())
	}
}

impl IterHandle for VariantTileHandle {
	fn iter_handle(&self) -> Box<dyn Iterator<Item = &Handle<Texture>> + '_> {
		Box::new(match self.tile {
			SimpleTileHandle::Standard(ref handle) => Box::new(std::slice::from_ref(handle).iter()),
			SimpleTileHandle::Animated(ref anim) => anim.iter_handle(),
		})
	}
}

impl IterHandle for AutoTileHandle {
	fn iter_handle(&self) -> Box<dyn Iterator<Item = &Handle<Texture>> + '_> {
		Box::new(
			self.variants
				.iter()
				.flat_map(|variant| variant.iter_handle()),
		)
	}
}

impl IterHandle for TileHandle {
	fn iter_handle(&self) -> Box<dyn Iterator<Item = &Handle<Texture>> + '_> {
		match self {
			Self::Standard(ref handle) => Box::new(std::slice::from_ref(handle).iter()),
			Self::Animated(ref anim) => anim.iter_handle(),
			Self::Variant(ref variants) => {
				Box::new(variants.iter().flat_map(|variant| variant.iter_handle()))
			}
			Self::Auto(ref autos) => Box::new(autos.iter().flat_map(|auto| auto.iter_handle())),
		}
	}
}
