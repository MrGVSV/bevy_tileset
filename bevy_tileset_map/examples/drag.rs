//! This example demonstrates how the [`TilePlacer`] can be used to create
//! a (very basic) dragging system using its placement methods and return values.

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_ecs_tilemap::{ChunkSize, MapQuery, MapSize, TilePos, TilemapPlugin};

use bevy_tileset_map::prelude::*;

mod helpers;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum MapState {
	LoadingTileset,
	BuildingMap,
	Ready,
}

fn main() {
	App::new()
		// === Required === //
		.add_plugins(DefaultPlugins)
		.add_plugin(TilemapPlugin)
		.add_plugin(TilesetPlugin)
		// /== Required === //
		// === Exmaple-Specific === //
		.add_startup_system(setup_hud)
		.add_state(MapState::LoadingTileset)
		.add_event::<helpers::ClickEvent>()
		.init_resource::<MyTileset>()
		.init_resource::<DragTool>()
		.add_system(helpers::set_texture_filters_to_nearest.label("helper"))
		.add_system(helpers::on_click)
		.add_system_set(
			SystemSet::on_enter(MapState::LoadingTileset)
				.with_system(load_tiles)
				.after("helper"),
		)
		.add_system_set(
			SystemSet::on_update(MapState::LoadingTileset).with_system(check_tiles_loaded),
		)
		.add_system_set(SystemSet::on_enter(MapState::BuildingMap).with_system(build_map))
		.add_system_set(SystemSet::on_update(MapState::Ready).with_system(on_drag))
		// /== Exmaple-Specific === //
		.run();
}

#[derive(Default)]
struct MyTileset {
	/// This stores the handle to our tileset so it doesn't get unloaded
	handle: Option<Handle<Tileset>>,
}

#[derive(Default, Debug)]
struct DragTool {
	last_pos: UVec2,
	is_placing: bool,
	is_active: bool,
}

fn on_drag(
	tilesets: Tilesets,
	my_tileset: Res<MyTileset>,
	mut drag_tool: ResMut<DragTool>,
	mut event_reader: EventReader<helpers::ClickEvent>,
	mut placer: TilePlacer,
	query: Query<&Transform, With<helpers::WorldCamera>>,
	windows: Res<Windows>,
) {
	let handle = my_tileset.handle.clone().unwrap();
	if let Some(tileset) = tilesets.get(handle) {
		// Control whether the drag tool is active or not
		let was_active = drag_tool.is_active;
		for helpers::ClickEvent(.., pressed) in event_reader.iter() {
			drag_tool.is_active = *pressed;
		}

		if drag_tool.is_active {
			// Dragging -> place/remove tiles

			if let Some(pos) = helpers::get_mouse_pos(&query, &windows) {
				if drag_tool.last_pos == pos && was_active {
					// Same tile coordinate as before -> skip
					return;
				}

				drag_tool.last_pos = pos;

				let pos: TilePos = pos.into();
				let tileset_id = tileset.id().clone();
				let layer_id = 0u16;

				if let Some(group_id) = tileset.get_tile_group_id("Wall") {
					// Get tile ID
					let tile_id = TileId::new(*group_id, tileset_id);

					if !was_active {
						// Check to see whether we should be placing or removing tiles
						// This is based on whether or not the first click adds or removes a tile
						let result = placer.toggle(tile_id, pos, 0u16, layer_id);
						drag_tool.is_placing = matches!(result, Ok(PlacedTile::Added { .. }));
					} else if drag_tool.is_placing {
						placer.try_place(tile_id, pos, 0u16, layer_id).ok();
					} else {
						placer.remove(pos, 0u16, layer_id).ok();
					}
				}
			}
		}
	}
}

/// A system used to build the tilemap
fn build_map(
	tilesets: Tilesets,
	my_tileset: Res<MyTileset>,
	mut commands: Commands,
	mut map_query: MapQuery,
	mut state: ResMut<State<MapState>>,
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

	state.set(MapState::Ready).unwrap();
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

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");

	commands.spawn_bundle(UiCameraBundle::default());
	commands.spawn_bundle(TextBundle {
		style: Style {
			align_self: AlignSelf::FlexEnd,
			justify_content: JustifyContent::FlexStart,
			position_type: PositionType::Absolute,
			position: Rect {
				top: Val::Px(15.0),
				left: Val::Px(15.0),
				..Default::default()
			},
			..Default::default()
		},
		text: Text::with_section(
			"Click and drag!",
			TextStyle {
				font,
				font_size: 32.0,
				color: Color::rgba(0.15, 0.15, 0.25, 0.85),
			},
			TextAlignment::default(),
		),
		..Default::default()
	});
}
