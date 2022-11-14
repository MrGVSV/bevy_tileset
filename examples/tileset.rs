//! This example demonstrates the most basic setup for loading a tileset
//!
//! Essentially, all we need to do is load the config file via the `AssetServer` and keep
//! a handle to the asset. That's it! From there, you can use the `Tilesets` system parameter
//! (or simply `Res<Assets<Tileset>>` if you prefer) to access the stored tile and texture atlas
//! data.

use bevy::prelude::*;

use bevy_tileset::prelude::*;

fn main() {
	App::new()
		// === Required === //
		.add_plugins(DefaultPlugins)
		.add_plugin(TilesetPlugin::default())
		// /== Required === //
		.init_resource::<MyTileset>()
		.add_startup_system(load_tileset)
		.add_system(show_tileset)
		.run();
}

#[derive(Resource, Default)]
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
	mut has_ran: Local<bool>,
) {
	if my_tileset.handle.is_none() || *has_ran || !tilesets.contains_name("My Awesome Tileset") {
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
		let texture = tileset.texture().clone();
		commands.spawn(Camera2dBundle::default());
		commands.spawn(SpriteBundle {
			texture,
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			..Default::default()
		});

		// === Display Tile === //
		if let Some((ref tile_index, ..)) = tileset.select_tile("Grass") {
			match tile_index {
				TileIndex::Standard(index) => {
					// Do something standard
					commands.spawn(SpriteSheetBundle {
						transform: Transform {
							translation: Vec3::new(08.0, -48.0, 0.0),
							..Default::default()
						},
						sprite: TextureAtlasSprite::new(*index),
						texture_atlas: atlas.clone(),
						..Default::default()
					});
				},
				TileIndex::Animated(start, end, speed) => {
					// Do something  ✨ animated ✨
				},
			}
		}

		*has_ran = true;
	}
}
