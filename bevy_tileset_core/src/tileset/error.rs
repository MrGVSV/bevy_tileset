use crate::prelude::TileGroupId;
use bevy::asset::AssetIoError;
use bevy::render::texture::TextureError;
use bevy_tile_atlas::TileAtlasBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TilesetError {
	#[error("image could not be found")]
	ImageNotFound,
	#[error("could not load asset: {0:?}")]
	AssetIoError(AssetIoError),
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
}
