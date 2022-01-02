use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use bevy::asset::{
	Asset, AssetLoader, AssetPath, BoxedFuture, Handle, HandleId, LoadContext, LoadedAsset,
};
use bevy::render::texture::{ImageType, Texture};
use bevy::utils::Uuid;
use bevy_ecs_tileset_tiles::prelude::{TileDef, TileHandle};
use bevy_tile_atlas::TextureStore;
use serde::{Deserialize, Serialize};

use crate::prelude::error::TilesetError;
use crate::prelude::TilesetBuilder;
use crate::tileset::load::{load_tile_handles, TextureLoader};
use crate::{TileGroupId, TilesetId};

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

struct TilesetTextureLoader<'x, 'y> {
	context: &'x mut LoadContext<'y>,
	images: Arc<RwLock<HashMap<HandleId, PathBuf>>>,
}

struct TilesetTextureServer<'x, 'y> {
	context: &'x mut LoadContext<'y>,
	images: HashMap<HandleId, Texture>,
	dep_count: u8,
}

impl<'x, 'y> TextureLoader for TilesetTextureLoader<'x, 'y> {
	fn load<'a, T: Asset, P: Into<AssetPath<'a>>>(&self, path: P) -> Handle<Texture> {
		let asset_path = path.into().clone();
		let handle: Handle<Texture> = self.context.get_handle(asset_path.clone());
		let path = asset_path.path().to_path_buf();

		if let Ok(mut images) = self.images.try_write() {
			images.insert(handle.id, path);
		}
		handle
	}
}

impl<'x, 'y> TilesetTextureLoader<'x, 'y> {
	fn collect(self) -> BoxedFuture<'x, Result<HashMap<HandleId, Texture>, TilesetError>> {
		let images = self.images.read().unwrap().clone();
		Box::pin(async move {
			let image_map = futures::future::join_all(
				images
					.into_iter()
					.map(|(id, path)| load_image(&self.context, id, path)),
			)
			.await
			.into_iter()
			.filter_map(|x| x.ok())
			.collect();

			Ok(image_map)
		})
	}
}

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

impl<'x, 'y> TextureStore for TilesetTextureServer<'x, 'y> {
	fn add(&mut self, asset: Texture) -> Handle<Texture> {
		let prefix = self.context.path().to_str().unwrap_or("UNKNOWN_TILESET");
		let label = format!("{:?}-{:?}", prefix, self.dep_count);
		self.dep_count += 1;
		self.context
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

			let loader = TilesetTextureLoader {
				images: Arc::new(RwLock::new(HashMap::new())),
				context: load_context,
			};

			let tile_handles = get_tile_handles(&loader, &config.tiles).await?;
			let image_map = loader.collect().await?;

			let mut server = TilesetTextureServer {
				context: load_context,
				images: image_map,
				dep_count: 0,
			};

			let mut builder = TilesetBuilder::default();
			for (group_id, tile_handle) in tile_handles {
				builder.add_tile(tile_handle, group_id, &server)?;
			}

			let name = config
				.name
				.unwrap_or_else(|| Uuid::new_v4().to_hyphenated().to_string());
			let tileset = builder.build(name, config.id, &mut server)?;

			load_context.set_default_asset(LoadedAsset::new(tileset));

			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["ron"]
	}
}

async fn get_tile_handles<'x, 'y>(
	loader: &'x TilesetTextureLoader<'x, 'y>,
	tile_paths: &BTreeMap<TileGroupId, String>,
) -> Result<Vec<(TileGroupId, TileHandle)>, TilesetError> {
	let tile_defs = futures::future::join_all(
		tile_paths
			.iter()
			.map(|(.., tile_path)| load_tile(&loader.context, tile_path)),
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
