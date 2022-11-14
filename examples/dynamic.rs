//! This example showcases how to create a dynamic tileset
//!
//! Tilesets can be created at runtime as opposed to being pre-defined in a config file.
//! This allows for greater control over which tiles get added to the tileset and which don't.
//!
//! One important note is the difference between a `Tileset` and a `RawTileset`. A `Tileset` is
//! generally your pre-defined tilesets. They're stored in an `Assets` resource and only contain
//! a handle to the internal `TextureAtlas`. On the other hand, a `RawTileset` is the dynamically
//! created one and provides immediate access to the internal `TextureAtlas`. This is useful for
//! using it directly, but doesn't work well for cases where a handle to that `TextureAtlas` is
//! needed. Luckily, you can convert it to a `Tileset` and add it to the `Assets<Tileset>` resource
//! any time.

use bevy::asset::FileAssetIo;
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
		.add_system(check_loaded)
		.add_system(show_tileset)
		.run();
}

#[derive(Resource)]
struct MyTileset {
	/// This stores the handle to our tileset so it doesn't get unloaded
	tiles: Option<Vec<TileHandle>>,
	/// This is the raw tileset (a tileset that was generated manually)
	raw_tileset: Option<RawTileset>,
	is_loaded: bool,
}

impl Default for MyTileset {
	fn default() -> Self {
		Self {
			tiles: None,
			is_loaded: false,
			raw_tileset: None,
		}
	}
}

/// Starts the tileset loading process
fn load_tileset(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
	// You can dynamically load the TileDef config files
	let asset_path = FileAssetIo::get_base_path().join("assets");
	let dirt_path = asset_path.join("tiles/dirt.ron");
	let glass_path = asset_path.join("tiles/glass.ron");

	let dirt_bytes = std::fs::read(dirt_path).unwrap();
	let glass_bytes = std::fs::read(glass_path).unwrap();

	let dirt_tile = ron::de::from_bytes::<TileDef>(&dirt_bytes).unwrap();
	let glass_tile = ron::de::from_bytes::<TileDef>(&glass_bytes).unwrap();

	// Automatically generate the TileHandle collection
	let mut handles = load_tile_handles(vec![dirt_tile, glass_tile], &asset_server);

	// You can also manually construct the TileHandle yourself
	let grass_handle: Handle<Image> = asset_server.load("tiles/grass.png");
	let grass_tile = TileHandle::new_standard("Dynamic Grass", grass_handle);
	handles.push(grass_tile);

	my_tileset.tiles = Some(handles);
}

fn check_loaded(
	mut my_tileset: ResMut<MyTileset>,
	asset_server: Res<AssetServer>,
	mut textures: ResMut<Assets<Image>>,
) {
	if my_tileset.is_loaded || my_tileset.tiles.is_none() {
		return;
	}

	// We MUST ensure that every handle is loaded
	let is_loaded = my_tileset
		.tiles
		.as_ref()
		.unwrap()
		.iter()
		.all(|tile| tile.is_loaded(&asset_server));

	if !is_loaded {
		return;
	}

	// Build the tileset
	let mut builder = TilesetBuilder::default();
	// We use a reference here because we still need to keep these strong handles loaded
	// (the RawTileset will only store weak handles)
	let tiles = my_tileset.tiles.as_ref().unwrap();
	for (group_id, tile) in tiles.iter().enumerate() {
		builder
			.add_tile(tile.clone(), group_id as TileGroupId, &textures)
			.unwrap();
	}

	let raw_tileset = builder
		.build("My Dynamic Tileset", 123, &mut textures)
		.unwrap();

	// We could also choose to add it to the `Assets<Tileset>` resource so we could use `Tilesets`, but we'll
	// just hold onto it manually for now.
	// If you did want to do that, you would simply generate the `Tileset` and add it to the `Assets<Tileset>` resource:
	// ```
	// let tileset = raw_tileset.into_asset(atlases_asset); // Where `atlases_asset` is a `Assets<TextureAtlas>` resource
	// let tileset_handle = tileset_assets.add(tileset);
	// ```

	my_tileset.raw_tileset = Some(raw_tileset);
	my_tileset.is_loaded = true;
}

/// Shows the tileset
fn show_tileset(
	mut commands: Commands,
	my_tileset: Res<MyTileset>,
	mut has_ran: Local<bool>,
	mut textures: ResMut<Assets<Image>>,
) {
	if my_tileset.raw_tileset.is_none() || *has_ran {
		return;
	}

	let raw_tileset = my_tileset.raw_tileset.as_ref().unwrap();
	println!("{:#?}", raw_tileset);

	// === Display Tileset === //
	let texture = raw_tileset.texture().clone();
	commands.spawn(Camera2dBundle::default());
	commands.spawn(SpriteBundle {
		texture,
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..Default::default()
	});

	// === Display Tile === //
	if let Some((ref tile_index, ..)) = raw_tileset.select_tile("Dynamic Grass") {
		match tile_index {
			TileIndex::Standard(index) => {
				// Do something standard
				if let Some(handle) = raw_tileset.get_tile_handle(index) {
					let mut texture = handle.clone();
					// Handles in the tileset are weak by default so we'll need to make it strong again so the image doesn't unload
					texture.make_strong(&mut textures);
					commands.spawn(SpriteBundle {
						texture,
						transform: Transform::from_xyz(0.0, 48.0, 0.0),
						..Default::default()
					});
				}
			},
			TileIndex::Animated(start, end, speed) => {
				// Do something  ✨ animated ✨
			},
		}
	}

	*has_ran = true;
}
