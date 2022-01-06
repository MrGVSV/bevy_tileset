use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use bevy::asset::{
	Asset, AssetLoader, AssetPath, BoxedFuture, Handle, HandleId, LoadContext, LoadedAsset,
};
use bevy::render::texture::{ImageType, Texture};
use bevy::utils::Uuid;
use bevy_tile_atlas::TextureStore;
use bevy_tileset_tiles::prelude::{TileDef, TileHandle};
use serde::{Deserialize, Serialize};

use crate::prelude::{TileGroupId, TilesetBuilder, TilesetError, TilesetId};
use crate::tileset::load::{load_tile_handles, TextureLoader};

#[derive(Default)]
pub struct TilesetAssetLoader;

#[derive(Default, Deserialize, Serialize)]
pub struct TilesetDef {
	/// The optional name of the tileset (defaults to a random UUID string)
	pub name: Option<String>,
	/// The ID of the tileset
	pub id: TilesetId,
	/// The tiles in this tileset as a mapping of their group ID to the relative path to
	/// their definition file
	pub tiles: BTreeMap<TileGroupId, String>,
}

/// A struct that mimics a Bevy `AssetServer`
///
/// Instead of loading an image right away, it tracks the paths to the images to be loaded
/// later (so we don't need to await on _every_ image).
struct TilesetTextureLoader<'x, 'y> {
	load_context: &'x mut LoadContext<'y>,
	/// The images that need to be loaded
	bytes: Arc<RwLock<HashMap<HandleId, PathBuf>>>,
}

/// A struct that mimics a Bevy `Assets<Texture>` resource by allowing get/add operations
struct TilesetTextureStore<'x, 'y> {
	load_context: &'x mut LoadContext<'y>,
	images: HashMap<HandleId, Texture>,
}

impl<'x, 'y> TextureLoader for TilesetTextureLoader<'x, 'y> {
	fn load_texture<'a, T: Asset, P: Into<AssetPath<'a>>>(&self, path: P) -> Handle<Texture> {
		let asset_path = path.into().clone();
		let handle: Handle<Texture> = self.load_context.get_handle(asset_path.clone());
		let path = asset_path.path().to_path_buf();

		if let Ok(mut images) = self.bytes.try_write() {
			images.insert(handle.id, path);
		}
		handle
	}
}

impl<'x, 'y> TilesetTextureLoader<'x, 'y> {
	/// Load the images and collect them into a HashMap
	fn collect_images(self) -> BoxedFuture<'x, Result<HashMap<HandleId, Texture>, TilesetError>> {
		let images = self.bytes.read().unwrap().clone();
		Box::pin(async move {
			let image_map = futures::future::join_all(
				images
					.into_iter()
					.map(|(id, path)| load_image(&self.load_context, id, path)),
			)
			.await
			.into_iter()
			.filter_map(|x| x.ok())
			.collect();

			Ok(image_map)
		})
	}
}

impl<'x, 'y> TextureStore for TilesetTextureStore<'x, 'y> {
	fn add(&mut self, asset: Texture) -> Handle<Texture> {
		//! This should only really be called once: When creating the tile texture atlas
		//! since we'll need to track that asset as well.
		let prefix = self
			.load_context
			.path()
			.to_str()
			.unwrap_or("UNKNOWN_TILESET");
		let label = format!("Tileset__[{:?}]__{:?}", prefix, Uuid::new_v4().to_string());
		self.load_context
			.set_labeled_asset(&label, LoadedAsset::new(asset))
	}

	fn get<H: Into<HandleId>>(&self, handle: H) -> Option<&Texture> {
		self.images.get(&handle.into())
	}
}

impl AssetLoader for TilesetAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
		Box::pin(async move {
			let config = ron::de::from_bytes::<TilesetDef>(bytes)?;

			// === Load Handles === //
			let loader = TilesetTextureLoader {
				bytes: Arc::new(RwLock::new(HashMap::new())),
				load_context,
			};

			let tile_handles = get_tile_handles(&loader, &config.tiles).await?;

			// === Build Tiles === //
			let images = loader.collect_images().await?;
			let mut store = TilesetTextureStore {
				load_context,
				images,
			};

			let mut builder = TilesetBuilder::default();
			for (group_id, tile_handle) in tile_handles {
				builder.add_tile(tile_handle, group_id, &store)?;
			}

			// === Finish Tileset === //
			let name = config
				.name
				.unwrap_or_else(|| Uuid::new_v4().to_hyphenated().to_string());
			let tileset = builder.build(name, config.id, &mut store)?;

			load_context.set_default_asset(LoadedAsset::new(tileset));

			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["ron"]
	}
}

/// Get a `Vec` of ([`TileGroupId`], [`TileHandle`]) tuples
async fn get_tile_handles<'x, 'y>(
	loader: &'x TilesetTextureLoader<'x, 'y>,
	tile_paths: &BTreeMap<TileGroupId, String>,
) -> Result<Vec<(TileGroupId, TileHandle)>, TilesetError> {
	let tile_defs = futures::future::join_all(
		tile_paths
			.iter()
			.map(|(.., tile_path)| load_tile(&loader.load_context, tile_path)),
	)
	.await
	.into_iter()
	.filter_map(|tile_def| tile_def.ok())
	.collect::<Vec<_>>();

	let handles = load_tile_handles(tile_defs, loader);

	Ok(tile_paths
		.iter()
		.map(|(id, ..)| *id)
		.zip(handles.into_iter().map(|handle| handle))
		.collect())
}

/// Load the tile definition at the given path and return its corresponding [TileDef]
///
/// The path is always relative to the tileset's configuration file path
async fn load_tile(context: &LoadContext<'_>, path: &str) -> Result<TileDef, TilesetError> {
	let path = if let Some(parent) = context.path().parent() {
		parent.join(path)
	} else {
		Path::new(path).to_path_buf()
	};
	let bytes = context
		.read_asset_bytes(&path)
		.await
		.map_err(|err| TilesetError::AssetIO(err))?;
	let def = ron::de::from_bytes::<TileDef>(&bytes)
		.map_err(|err| TilesetError::InvalidDefinition(err))?;
	Ok(def)
}

/// Load an image at the given path
async fn load_image(
	context: &LoadContext<'_>,
	id: HandleId,
	path: PathBuf,
) -> Result<(HandleId, Texture), TilesetError> {
	let bytes = context
		.read_asset_bytes(path.clone())
		.await
		.map_err(|err| TilesetError::AssetIO(err))?;
	let path = path.as_path();
	let ext = path.extension().unwrap().to_str().unwrap();
	let img = Texture::from_buffer(&bytes, ImageType::Extension(ext))
		.map_err(|err| TilesetError::ImageLoad(err))?;
	Ok((id, img))
}
