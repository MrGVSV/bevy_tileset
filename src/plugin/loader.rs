//! Module used for loading and creating tilesets
//!
//! Currently, all generated tilesets are stored in the [`Tilesets`] resource,
//! where they may be accessed by name directly once loaded

use bevy::log::warn;
use std::collections::HashMap;
use std::fs::DirEntry;

use crate::handles::TilesetHandles;
use crate::prelude::{TileDef, TilesetBuilder};
use crate::Tilesets;
use bevy::prelude::{AssetServer, Assets, EventReader, EventWriter, Res, ResMut, Texture};
use bevy::utils::Uuid;

/// The default assets directory path where all tiles should be defined
pub const DEFAULT_TILES_ASSET_DIR: &str = "tiles";

pub enum TilesetLoadEvent {
	LoadTiles(TilesetLoader),
	GeneratedTileset(String),
}

#[derive(Default)]
pub(crate) struct TilesetHandlesMap(HashMap<String, TilesetRequest>);

#[derive(Default)]
struct TilesetRequest {
	handles: TilesetHandles,
	max_columns: Option<usize>,
}

/// A structure defining how the tileset should be loaded
pub struct TilesetLoader {
	/// The name of this Tileset
	///
	/// This is mainly used for identifying tilesets after generation
	pub name: String,
	pub dirs: Vec<TilesetDirs>,
	pub max_columns: Option<usize>,
}

/// Directories for the tileset to be loaded
pub struct TilesetDirs {
	/// The asset directory containing the tile definitions
	///
	/// Default: [`DEFAULT_TILES_ASSET_DIR`]
	pub tile_directory: String,

	/// The asset directory containing the tile textures
	///
	/// Default: [`DEFAULT_TILES_ASSET_DIR`]
	pub texture_directory: String,
}

impl TilesetLoader {
	pub fn named(name: &str, dirs: Vec<TilesetDirs>) -> Self {
		Self {
			name: name.to_string(),
			dirs,
			max_columns: None,
		}
	}

	pub fn unnamed(dirs: Vec<TilesetDirs>) -> Self {
		Self {
			name: get_unique_name(),
			dirs,
			max_columns: None,
		}
	}
}

impl Default for TilesetLoader {
	fn default() -> Self {
		Self {
			name: get_unique_name(),
			dirs: vec![TilesetDirs::default()],
			max_columns: Default::default(),
		}
	}
}

impl From<TilesetLoader> for TilesetLoadEvent {
	fn from(loader: TilesetLoader) -> Self {
		TilesetLoadEvent::LoadTiles(loader)
	}
}

impl Default for TilesetDirs {
	fn default() -> Self {
		Self {
			tile_directory: DEFAULT_TILES_ASSET_DIR.to_string(),
			texture_directory: DEFAULT_TILES_ASSET_DIR.to_string(),
		}
	}
}

impl From<&str> for TilesetDirs {
	fn from(dir: &str) -> Self {
		TilesetDirs::from_dir(dir)
	}
}

impl From<(&str, &str)> for TilesetDirs {
	fn from(dirs: (&str, &str)) -> Self {
		TilesetDirs::from_dirs(dirs.0, dirs.1)
	}
}

impl TilesetDirs {
	pub fn from_dir(tile_directory: &str) -> Self {
		Self {
			tile_directory: tile_directory.to_string(),
			texture_directory: tile_directory.to_string(),
		}
	}

	pub fn from_dirs(tile_directory: &str, texture_directory: &str) -> Self {
		Self {
			tile_directory: tile_directory.to_string(),
			texture_directory: texture_directory.to_string(),
		}
	}
}

/// __\[SYSTEM\]__ Loads the tiles (on event)
pub(crate) fn on_load_tileset_event(
	mut events: EventReader<TilesetLoadEvent>,
	mut handles_map: ResMut<TilesetHandlesMap>,
	asset_server: Res<AssetServer>,
) {
	for event in events.iter() {
		if let TilesetLoadEvent::LoadTiles(ref loader) = event {
			load_tiles(loader, &mut handles_map, &asset_server);
		}
	}
}

/// __\[SYSTEM\]__ Creates the tileset once all tiles are loaded and sends it out as an event
pub(crate) fn create_tileset(
	mut handles_map: ResMut<TilesetHandlesMap>,
	mut tilesets: ResMut<Tilesets>,
	mut textures: ResMut<Assets<Texture>>,
	mut events_writer: EventWriter<TilesetLoadEvent>,
	asset_server: Res<AssetServer>,
) {
	handles_map.0.retain(|tileset_name, tileset_request| {
		let tileset_handles = &tileset_request.handles;

		if tileset_handles.len() == 0usize {
			return false;
		}

		if !tileset_handles.is_dirty {
			// No update needed
			return false;
		}

		if !tileset_handles.is_loaded(&asset_server) {
			// Textures have yet to be fully loaded
			return true;
		}

		let id = tilesets.next_id();
		let mut builder = TilesetBuilder::default();
		builder.add_handles(tileset_handles, &textures);
		if let Ok(tileset) = builder.build(tileset_name.clone(), id, &mut textures) {
			tilesets.register(tileset);
			events_writer.send(TilesetLoadEvent::GeneratedTileset(tileset_name.clone()));
		}

		false
	});
}

fn load_tiles(
	loader: &TilesetLoader,
	handles_map: &mut ResMut<TilesetHandlesMap>,
	asset_server: &Res<AssetServer>,
) {
	let tileset_name = if loader.name.is_empty() {
		get_unique_name()
	} else {
		loader.name.clone()
	};

	let request = handles_map
		.0
		.entry(tileset_name)
		.or_insert_with(TilesetRequest::default);

	request.max_columns = loader.max_columns;

	for TilesetDirs {
		ref tile_directory,
		ref texture_directory,
	} in &loader.dirs
	{
		// === Load Config Files === //
		let dir = ::std::fs::read_dir(format!("assets/{}", tile_directory))
			.unwrap_or_else(|_| panic!("Could not find directory `{}`", tile_directory));

		let config_files = dir.filter_map::<DirEntry, _>(Result::ok).filter(|file| {
			if let Some(ext) = file.path().extension() {
				return ext == "ron";
			}
			false
		});

		// === Load Handles === //
		for config_file in config_files {
			let bytes = ::std::fs::read(config_file.path()).unwrap();
			let tile_def = ron::de::from_bytes::<TileDef>(bytes.as_slice());

			if let Ok(tile_def) = tile_def {
				request
					.handles
					.add_tile(tile_def, texture_directory, asset_server);
			} else if let Err(err) = tile_def {
				warn!(
					"Failed to load tile: {:?} ({:?} @ {:?})",
					config_file.path(),
					err.code,
					err.position
				);
			}
		}

		request.handles.is_dirty = true;
	}
}

fn get_unique_name() -> String {
	Uuid::new_v4().to_hyphenated().to_string()
}
