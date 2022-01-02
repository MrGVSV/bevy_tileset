use bevy::asset::AssetIoError;
use bevy::render::texture::TextureError;
use bevy_tile_atlas::TileAtlasBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TilesetError {
	#[error("image could not be found")]
	ImageNotFound,
	#[error("could not load asset: {0:?}")]
	AssetIO(AssetIoError),
	#[error("could not read image: {0:?}")]
	ImageLoad(TextureError),
	#[error("could not add tile to atlas: {0:?}")]
	Atlas(TileAtlasBuilderError),
	#[error("invalid tile data (expected {expected:?}, found {found:?})")]
	InvalidData { expected: String, found: String },
	#[error("could not read tile definition file: {0:?}")]
	InvalidDefinition(ron::Error),
}
