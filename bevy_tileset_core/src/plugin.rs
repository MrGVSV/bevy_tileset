use crate::tileset::{Tileset, TilesetAssetLoader, TilesetMap};
use bevy::prelude::*;

/// Plugin for setting up tilesets
#[derive(Default)]
pub struct TilesetPlugin {}

impl Plugin for TilesetPlugin {
	fn build(&self, app: &mut App) {
		app.init_asset_loader::<TilesetAssetLoader>()
			.init_asset::<Tileset>()
			.init_resource::<TilesetMap>()
			.add_systems(Update, tileset_event_sys);
	}
}

/// System that registers/deregisters tilesets as they are loaded and unloaded
fn tileset_event_sys(
	mut event_reader: EventReader<AssetEvent<Tileset>>,
	mut map: ResMut<TilesetMap>,
	tilesets: Res<Assets<Tileset>>,
	asset_server: ResMut<AssetServer>,
) {
	for event in event_reader.read() {
		match event {
			AssetEvent::<Tileset>::Added { id } => {
				if let Some(handle) = asset_server.get_id_handle(*id) {
					if let Some(tileset) = tilesets.get(handle.clone()) {
						map.register_tileset(tileset, &handle);
					}
				}
			},
			AssetEvent::<Tileset>::Removed { id } => {
				if let Some(handle) = asset_server.get_id_handle(*id) {
					map.deregister_tileset(&handle);
				}
			},
			_ => {},
		}
	}
}
