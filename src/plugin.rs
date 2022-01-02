use crate::tileset::{Tileset, TilesetAssetLoader, TilesetMap};
use bevy::app::AppBuilder;
use bevy::prelude::*;
use bevy_ecs_tilemap::{TilemapPlugin, TilemapStage};

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub struct TilesetStage;

#[derive(SystemLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TilesetLabel {
	/// Labels the system that handles auto tile updates
	UpdateAutoTiles,
	/// Labels the system that handles auto tile removals
	RemoveAutoTiles,
}

/// Plugin for setting up tilesets
pub struct TilesetPlugin;

impl Plugin for TilesetPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_plugin(TilemapPlugin)
			.add_stage_before(TilemapStage, TilesetStage, SystemStage::parallel())
			.add_asset::<Tileset>()
			.init_asset_loader::<TilesetAssetLoader>()
			.init_resource::<TilesetMap>()
			.add_system(tileset_event_sys.system());

		#[cfg(feature = "auto-tile")]
		app.add_event::<crate::prelude::RemoveAutoTileEvent>()
			.add_system_set_to_stage(
				TilesetStage,
				SystemSet::new().with_system(
					crate::auto::on_remove_auto_tile
						.system()
						.label(TilesetLabel::RemoveAutoTiles),
				),
			)
			.add_system_to_stage(
				TilemapStage,
				crate::auto::on_change_auto_tile
					.system()
					.label(TilesetLabel::UpdateAutoTiles)
					.before("update_chunk_visibility"),
			);
	}
}

fn tileset_event_sys(
	mut event_reader: EventReader<AssetEvent<Tileset>>,
	mut map: ResMut<TilesetMap>,
	tilesets: Res<Assets<Tileset>>,
) {
	for event in event_reader.iter() {
		match event {
			AssetEvent::<Tileset>::Created { handle } => {
				if let Some(tileset) = tilesets.get(handle.id) {
					map.add_tileset(tileset, &handle);
				}
			}
			AssetEvent::<Tileset>::Removed { handle } => {
				map.remove_tileset(&handle);
			}
			_ => {}
		}
	}
}
