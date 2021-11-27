mod helpers;

use bevy::prelude::*;

use bevy_ecs_tilemap::{GPUAnimated, MapQuery, Tile};
use bevy_ecs_tilemap_tileset::debug::DebugTilesetPlugin;
use bevy_ecs_tilemap_tileset::prelude::*;

const MY_TILESET: &str = "My Tileset";

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(TilesetPlugin)
		.add_plugin(DebugTilesetPlugin::single_with_position(
			MY_TILESET,
			Vec3::new(192.0, -32.0, 1.0),
		))
		.add_event::<helpers::ClickEvent>()
		.insert_resource(BuildMode {
			tile_name: String::from("Wall"),
			active_layer: 0u16,
		})
		.add_startup_system(load_tiles.system())
		.add_startup_system(setup_hud.system())
		.add_system(build_map.system())
		.add_system(on_keypress.system())
		.add_system(helpers::on_click.system())
		.add_system(update_text.system())
		.add_system_to_stage(
			TilesetStage,
			on_tile_click.system().before(TilesetLabel::RemoveAutoTiles),
		)
		.run();
}

fn load_tiles(mut writer: EventWriter<TilesetLoadEvent>) {
	writer.send(TilesetLoadRequest::named(MY_TILESET, vec![TilesetDirs::from_dir("tiles")]).into());
}

#[derive(Default)]
struct BuildMapState {
	built: bool,
}

fn build_map(
	tilesets: Res<Tilesets>,
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut map_query: MapQuery,
	mut local_state: Local<BuildMapState>,
) {
	if local_state.built {
		// Only build the map once
		return;
	}

	if let Some(tileset) = tilesets.get_by_name(MY_TILESET) {
		println!("My Tileset: {:#?}", tileset);

		let map_size = UVec2::new(4, 4);
		let chunk_size = UVec2::new(5, 5);
		let layer_count = 3;
		let default_tile = match tileset.get_tile_index("Empty").unwrap() {
			TileIndex::Standard(index) => index,
			TileIndex::Animated(start, ..) => start,
		} as u16;
		let _map = helpers::build_map(
			tileset,
			map_size,
			chunk_size,
			layer_count,
			default_tile,
			&mut materials,
			&mut commands,
			&mut map_query,
		);

		local_state.built = true;
	}
}

#[derive(Debug)]
struct BuildMode {
	tile_name: String,
	active_layer: u16,
}

fn on_tile_click(
	tilesets: Res<Tilesets>,
	build_mode: Res<BuildMode>,
	query: Query<(&Tile, Option<&AutoTile>, Option<&GPUAnimated>)>,
	mut event_writer: EventWriter<RemoveAutoTileEvent>,
	mut event_reader: EventReader<helpers::ClickEvent>,
	mut map_query: MapQuery,
	mut commands: Commands,
) {
	if let Some(tileset) = tilesets.get_by_name(MY_TILESET) {
		for helpers::ClickEvent(ref pos) in event_reader.iter() {
			let mut was_removed = false;
			let layer_id = build_mode.active_layer;
			let tile_name = &build_mode.tile_name;
			let pos = *pos;

			if let Ok(entity) = map_query.get_tile_entity(pos, 0u16, layer_id) {
				if let Ok((tile, auto, ..)) = query.get(entity) {
					let name = tileset.get_tile_name_by_index(&(tile.texture_index as usize));
					if Some(tile_name) == name {
						// Tiles match --> remove
						tileset.place_tile(
							"Empty",
							pos,
							0u16,
							layer_id,
							&mut commands,
							&mut map_query,
						);
						was_removed = true;
					}
					if auto.is_some() {
						event_writer.send(RemoveAutoTileEvent(entity));
					}
				}
			}

			if !was_removed {
				tileset.place_tile(
					tile_name,
					pos,
					0u16,
					layer_id,
					&mut commands,
					&mut map_query,
				);
			}
		}
	}
}

fn on_keypress(keys: Res<Input<KeyCode>>, mut build_mode: ResMut<BuildMode>) {
	if keys.just_pressed(KeyCode::W) {
		build_mode.tile_name = String::from("Wall");
	} else if keys.just_pressed(KeyCode::G) {
		build_mode.tile_name = String::from("Glass");
	} else if keys.just_pressed(KeyCode::D) {
		build_mode.tile_name = String::from("Dirt");
	} else if keys.just_pressed(KeyCode::E) {
		build_mode.tile_name = String::from("Empty");
	} else if keys.just_pressed(KeyCode::P) {
		build_mode.tile_name = String::from("Pipe");
	} else if keys.just_pressed(KeyCode::Key1) {
		build_mode.active_layer = 0u16;
	} else if keys.just_pressed(KeyCode::Key2) {
		build_mode.active_layer = 1u16;
	} else if keys.just_pressed(KeyCode::Key3) {
		build_mode.active_layer = 2u16;
	}
}

struct HudText;
fn update_text(
	mut query: Query<&mut Text, With<HudText>>,
	tilesets: Res<Tilesets>,
	build_mode: Res<BuildMode>,
) {
	for mut text in query.iter_mut() {
		text.sections[1].value = format!("{}", tilesets.get_by_name(MY_TILESET).is_some());
		text.sections[4].value = build_mode.tile_name.to_string();
		text.sections[7].value = format!("{}", build_mode.active_layer + 1);
		text.sections[9].value = String::from("3");
	}
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font = asset_server.load("fonts/FiraMono-Medium.ttf");
	let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

	let style_key = TextStyle {
		font: font_bold,
		font_size: 20.0,
		color: Color::rgba(0.15, 0.15, 0.15, 0.75),
	};
	let style_value = TextStyle {
		font: font.clone(),
		font_size: 18.0,
		color: Color::rgba(0.75, 0.75, 0.75, 0.75),
	};
	let style_small = TextStyle {
		font,
		font_size: 14.0,
		color: Color::rgba(0.75, 0.75, 0.75, 0.65),
	};

	commands.spawn_bundle(UiCameraBundle::default());
	commands
		.spawn_bundle(TextBundle {
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
			text: Text {
				sections: vec![
					TextSection {
						value: "Tileset Loaded : ".to_string(),
						style: style_key.clone(),
					},
					TextSection {
						value: "false".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: "\n".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: "Tile : ".to_string(),
						style: style_key.clone(),
					},
					TextSection {
						value: "-".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: "\n".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: "Layer : ".to_string(),
						style: style_key.clone(),
					},
					TextSection {
						value: "-".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: " / ".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: "-".to_string(),
						style: style_value.clone(),
					},
					TextSection {
						value: "\n".to_string(),
						style: style_value,
					},
					TextSection {
						value: "Options :\n".to_string(),
						style: style_key.clone(),
					},
					TextSection {
						value: "  ( 1 ) Edit Layer 1\n".to_string(),
						style: style_small.clone(),
					},
					TextSection {
						value: "  ( 2 ) Edit Layer 2\n".to_string(),
						style: style_small.clone(),
					},
					TextSection {
						value: "  ( 3 ) Edit Layer 3\n".to_string(),
						style: style_small.clone(),
					},
					TextSection {
						value: "  ( w ) Set tile to 'Wall'\n".to_string(),
						style: style_small.clone(),
					},
					TextSection {
						value: "  ( g ) Set tile to 'Glass'\n".to_string(),
						style: style_small.clone(),
					},
					TextSection {
						value: "  ( d ) Set tile to 'Dirt'\n".to_string(),
						style: style_small.clone(),
					},
					TextSection {
						value: "  ( e ) Set tile to 'Empty'\n".to_string(),
						style: style_small,
					},
					TextSection {
						value: "\nClick to add/remove tiles".to_string(),
						style: style_key,
					},
				],
				alignment: Default::default(),
			},
			..Default::default()
		})
		.insert(HudText);
}
