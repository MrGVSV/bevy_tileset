use crate::tileset::{Tileset, TilesetAssetLoader, TilesetMap};
use bevy::app::AppBuilder;
use bevy::prelude::*;
/// Plugin for setting up tilesets
#[derive(Default)]
pub struct TilesetPlugin {}

impl Plugin for TilesetPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_asset::<Tileset>()
			.init_asset_loader::<TilesetAssetLoader>()
			.init_resource::<TilesetMap>()
			.add_system(tileset_event_sys.system());
	}
}

/// System that registers/deregisters tilesets as they are loaded and unloaded
fn tileset_event_sys(
	mut event_reader: EventReader<AssetEvent<Tileset>>,
	mut map: ResMut<TilesetMap>,
	tilesets: Res<Assets<Tileset>>,
) {
	for event in event_reader.iter() {
		match event {
			AssetEvent::<Tileset>::Created { handle } => {
				if let Some(tileset) = tilesets.get(handle.id) {
					map.register_tileset(tileset, &handle);
				}
			}
			AssetEvent::<Tileset>::Removed { handle } => {
				map.deregister_tileset(&handle);
			}
			_ => {}
		}
	}
}
