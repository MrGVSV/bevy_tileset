use bevy::prelude::*;
use bevy_tileset::prelude::*;

fn main() {
	App::build()
		// === Required === //
		.add_plugins(DefaultPlugins)
		.add_plugin(TilesetPlugin::default())
		// /== Required === //
		.init_resource::<MyTileset>()
		.add_startup_system(load_tileset.system())
		.add_system(show_tileset.system())
		.run();
}

#[derive(Default)]
struct MyTileset {
	/// This stores the handle to our tileset so it doesn't get unloaded
	handle: Option<Handle<Tileset>>,
}

/// Starts the tileset loading process
fn load_tileset(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
	my_tileset.handle = Some(asset_server.load("tilesets/my_tileset.ron"));
}

/// Shows the tileset
///
/// This uses the `Tilesets` system parameter. Internally it gets the `Res<Assets<Tileset>>`, but also provides
/// additional niceties (specifically fetching a tileset by name or ID).
fn show_tileset(
	tilesets: Tilesets,
	mut commands: Commands,
	my_tileset: Res<MyTileset>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut has_ran: Local<bool>,
) {
	if my_tileset.handle.is_none() || *has_ran {
		return;
	}

	let handle = my_tileset.handle.as_ref().unwrap();
	if let Some(_) = tilesets.get(handle) {
		println!("Got tileset by handle! ({:?})", my_tileset.handle);
	}
	if let Some(tileset) = tilesets.get_by_id(&0) {
		println!("Got tileset by ID! ({})", tileset.id());
	}
	if let Some(tileset) = tilesets.get_by_name("My Awesome Tileset") {
		println!("Got tileset by name! ({})", tileset.name());
		println!("{:#?}", tileset);

		// === Display Tileset === //
		let atlas = tileset.atlas();
		let texture = atlas.texture.clone();
		commands.spawn_bundle(OrthographicCameraBundle::new_2d());
		commands.spawn_bundle(SpriteBundle {
			material: materials.add(texture.into()),
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..Default::default()
		});

		// === Display Tile === //
		if let Some((ref tile_index, ..)) = tileset.select_tile("Grass") {
			match tile_index {
				TileIndex::Standard(index) => {
					// Do something standard
				}
				TileIndex::Animated(start, end, speed) => {
					// Do something  ✨ animated ✨
				}
			}
		}

		*has_ran = true;
	}
}
