use bevy::prelude::*;
use bevy_ecs_tilemap::{TilemapLabel, TilemapStage};
use bevy_tileset::prelude::TilesetPlugin as BasePlugin;

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
#[derive(Default)]
pub struct TilesetPlugin;

impl Plugin for TilesetPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(BasePlugin::default()).add_stage_before(
			TilemapStage,
			TilesetStage,
			SystemStage::parallel(),
		);

		#[cfg(feature = "auto-tile")]
		app.add_event::<crate::auto::RemoveAutoTileEvent>()
			.add_system_set_to_stage(
				TilesetStage,
				SystemSet::new().with_system(
					crate::auto::on_remove_auto_tile.label(TilesetLabel::RemoveAutoTiles),
				),
			)
			.add_system_to_stage(
				TilemapStage,
				crate::auto::on_change_auto_tile
					.label(TilesetLabel::UpdateAutoTiles)
					.before(TilemapLabel::UpdateChunkVisibility),
			);
	}
}
