use crate::plugin::loader::{create_tileset, on_load_tileset_event, TilesetHandlesMap};
use crate::prelude::TilesetLoadEvent;
use crate::tiles::auto_tile::{on_change_auto_tile, on_remove_auto_tile, RemoveAutoTileEvent};
use bevy::app::AppBuilder;
use bevy::prelude::*;
use bevy_ecs_tilemap::{TilemapPlugin, TilemapStage};

use crate::Tilesets;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub struct TilesetStage;

#[derive(SystemLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TilesetLabel {
	/// Labels the system that loads the tiles from the file system
	LoadTileset,
	/// Labels the system that generates the tilesets
	CreateTileset,
	/// Labels the system that handles auto tile updates
	UpdateAutoTiles,
	/// Labels the system that handles auto tile removals
	RemoveAutoTiles,
	/// Labels the system set containing all other labeled systems
	All,
}

/// Plugin for setting up tilesets
pub struct TilesetPlugin;

impl Plugin for TilesetPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_plugin(TilemapPlugin)
			.init_resource::<TilesetHandlesMap>()
			.init_resource::<Tilesets>()
			.add_stage_before(TilemapStage, TilesetStage, SystemStage::parallel())
			.add_event::<TilesetLoadEvent>()
			.add_event::<RemoveAutoTileEvent>()
			.add_system_set_to_stage(
				TilesetStage,
				SystemSet::new()
					.label(TilesetLabel::All)
					.with_system(
						on_load_tileset_event
							.system()
							.label(TilesetLabel::LoadTileset),
					)
					.with_system(
						create_tileset
							.system()
							.label(TilesetLabel::CreateTileset)
							.after(TilesetLabel::LoadTileset),
					)
					.with_system(
						on_remove_auto_tile
							.system()
							.label(TilesetLabel::RemoveAutoTiles),
					),
			)
			.add_system_to_stage(
				TilemapStage,
				on_change_auto_tile
					.system()
					.label(TilesetLabel::UpdateAutoTiles)
					.before("update_chunk_visibility"),
			);
	}
}
