use crate::prelude::TileGroupId;
use bevy::{
	asset::{AssetLoadError, ReadAssetBytesError},
	render::texture::TextureError,
};
use bevy_tile_atlas::TileAtlasBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TilesetError {
	#[error("image could not be found")]
	ImageNotFound,
	#[error("could not load asset: {0:?}")]
	IoError(std::io::Error),
	#[error("could not load asset: {0:?}")]
	AssetLoadError(AssetLoadError),
	#[error("could not load asset: {0:?}")]
	ReadAssetBytesError(ReadAssetBytesError),
	#[error("could not read image: {0:?}")]
	ImageError(TextureError),
	#[error("could not add tile to atlas: {0:?}")]
	AtlasError(TileAtlasBuilderError),
	#[error("invalid tile data (expected {expected:?}, found {found:?})")]
	InvalidData { expected: String, found: String },
	#[error("could not read tile definition file: {0:?}")]
	InvalidDefinition(ron::error::SpannedError),
	#[error("tile with group ID {0:?} already exists in the tileset")]
	TileAlreadyExists(TileGroupId),
	#[error("could not build tile atlas: {0:?}")]
	TileAtlasBuilderError(TileAtlasBuilderError),
}

impl From<TileAtlasBuilderError> for TilesetError {
	fn from(value: TileAtlasBuilderError) -> Self { Self::TileAtlasBuilderError(value) }
}

impl From<ron::error::SpannedError> for TilesetError {
	fn from(value: ron::error::SpannedError) -> Self { Self::InvalidDefinition(value) }
}

impl From<std::io::Error> for TilesetError {
	fn from(value: std::io::Error) -> Self { Self::IoError(value) }
}
