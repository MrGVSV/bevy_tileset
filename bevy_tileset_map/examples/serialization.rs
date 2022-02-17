//! This example shows what loading a serialized tilemap might look like using the `TilemapSerializer`
//! system parameter.

use std::fs;

use bevy::asset::{FileAssetIo, LoadState};
use bevy::prelude::*;
use bevy_ecs_tilemap::{ChunkSize, MapQuery, MapSize, TilemapLabel, TilemapPlugin};

use bevy_tileset_map::prelude::*;

mod helpers;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum MapState {
	LoadingTileset,
	BuildingMap,
	LoadingMap,
}

fn main() {
	App::new()
		// === Required === //
		.add_plugins(DefaultPlugins)
		.add_plugin(TilemapPlugin)
		.add_plugin(TilesetPlugin)
		// /== Required === //
		// === Exmaple-Specific === //
		.add_state(MapState::LoadingTileset)
		.add_state_to_stage(CoreStage::PostUpdate, MapState::LoadingTileset)
		.init_resource::<MyTileset>()
		.add_system(helpers::set_texture_filters_to_nearest.label("helper"))
		.add_system_set(
			SystemSet::on_enter(MapState::LoadingTileset)
				.with_system(load_tiles)
				.after("helper"),
		)
		.add_system_set(
			SystemSet::on_update(MapState::LoadingTileset).with_system(check_tiles_loaded),
		)
		.add_system_set(SystemSet::on_enter(MapState::BuildingMap).with_system(build_map))
		.add_system_set(SystemSet::on_update(MapState::BuildingMap).with_system(can_load_maps))
		.add_system_set(
			SystemSet::on_enter(MapState::LoadingMap)
				.with_system(load_maps)
				.after(TilemapLabel::UpdateChunkMesh),
		)
		// /== Exmaple-Specific === //
		.run();
}

#[derive(Default)]
struct MyTileset {
	/// This stores the handle to our tileset so it doesn't get unloaded
	handle: Option<Handle<Tileset>>,
}

/// Starts the tileset loading process
fn load_tiles(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
	my_tileset.handle = Some(asset_server.load("tilesets/my_tileset.ron"));
}

/// A system used to check the load state of the tileset
fn check_tiles_loaded(
	my_tileset: Res<MyTileset>,
	asset_server: Res<AssetServer>,
	mut state: ResMut<State<MapState>>,
) {
	if let Some(handle) = &my_tileset.handle {
		if LoadState::Loaded == asset_server.get_load_state(handle) {
			state.set(MapState::BuildingMap).unwrap();
		}
	}
}

/// A system used to build the tilemap
fn build_map(
	tilesets: Tilesets,
	my_tileset: Res<MyTileset>,
	mut commands: Commands,
	mut map_query: MapQuery,
) {
	let handle = my_tileset.handle.as_ref().unwrap();
	let tileset = tilesets.get(handle).unwrap();

	// === Settings === //
	let map_size = MapSize(4, 4);
	let chunk_size = ChunkSize(5, 5);
	let layer_count = 3;

	// === Build === //
	helpers::build_map(
		tileset,
		map_size,
		chunk_size,
		layer_count,
		&mut commands,
		&mut map_query,
	);
}

/// Ensures that the tilemaps has been fully generated before loading the saved maps
///
/// This works by waiting a frame so that bevy_ecs_tilemap can get caught up and the Commands can be flushed
fn can_load_maps(mut can_load: Local<bool>, mut state: ResMut<State<MapState>>) {
	if *can_load {
		state.set(MapState::LoadingMap).unwrap();
	}
	*can_load = true;
}

/// A system used to load the saved tilemaps from disk
///
/// The `TilemapSerializer` is a special system param that allows for entire tilemaps to be saved and loaded. Here,
/// we are using it to load a JSON file containing our already saved data.
fn load_maps(mut serializer: TilemapSerializer) {
	let path = FileAssetIo::get_root_path().join("assets/map.json");
	let data = fs::read_to_string(path).unwrap();
	let maps = serde_json::from_str::<SerializableTilemap>(&data).unwrap();

	serializer.load_maps(&maps);
}
