use bevy::asset::FileAssetIo;
use bevy::prelude::*;
use bevy_tileset::prelude::*;
use bevy_tileset::tiles::prelude::*;

fn main() {
	App::build()
		// === Required === //
		.add_plugins(DefaultPlugins)
		.add_plugin(TilesetPlugin::default())
		// /== Required === //
		.init_resource::<MyTileset>()
		.add_startup_system(load_tileset.system())
		.add_system(check_loaded.system())
		.add_system(show_tileset.system())
		.run();
}

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
	let asset_path = FileAssetIo::get_root_path().join("assets");
	let dirt_path = asset_path.join("tiles/dirt.ron");
	let glass_path = asset_path.join("tiles/glass.ron");

	let dirt_bytes = std::fs::read(dirt_path).unwrap();
	let glass_bytes = std::fs::read(glass_path).unwrap();

	let dirt_tile = ron::de::from_bytes::<TileDef>(&dirt_bytes).unwrap();
	let glass_tile = ron::de::from_bytes::<TileDef>(&glass_bytes).unwrap();

	// Automatically generate the TileHandle collection
	let mut handles = load_tile_handles(vec![dirt_tile, glass_tile], &asset_server);

	// You can also manually construct the TileHandle yourself
	let grass_handle: Handle<Texture> = asset_server.load("tiles/grass.png");
	let grass_tile = TileHandle::new_standard("Dynamic Grass", grass_handle);
	handles.push(grass_tile);

	my_tileset.tiles = Some(handles);
}

fn check_loaded(
	mut my_tileset: ResMut<MyTileset>,
	asset_server: Res<AssetServer>,
	mut textures: ResMut<Assets<Texture>>,
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
	let tiles = my_tileset.tiles.take();
	for (group_id, tile) in tiles.unwrap().into_iter().enumerate() {
		builder
			.add_tile(tile, group_id as TileGroupId, &textures)
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
	my_tileset.tiles = None;
	my_tileset.is_loaded = true;
}

/// Shows the tileset
///
/// This uses the `Tilesets` system parameter. Internally it gets the `Res<Assets<Tileset>>`, but also provides
/// additional niceties (specifically fetching a tileset by name or ID).
fn show_tileset(
	mut commands: Commands,
	my_tileset: Res<MyTileset>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut has_ran: Local<bool>,
) {
	if my_tileset.raw_tileset.is_none() || *has_ran {
		return;
	}

	let raw_tileset = my_tileset.raw_tileset.as_ref().unwrap();
	println!("{:#?}", raw_tileset);

	// === Display Tileset === //
	let atlas = raw_tileset.atlas();
	let texture = raw_tileset.texture().clone();
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(SpriteBundle {
		material: materials.add(texture.into()),
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..Default::default()
	});

	// === Display Tile === //
	if let Some((ref tile_index, ..)) = raw_tileset.select_tile("Dynamic Grass") {
		match tile_index {
			TileIndex::Standard(index) => {
				// Do something standard
				if let Some(handle) = raw_tileset.get_tile_handle(index) {
					commands.spawn_bundle(SpriteBundle {
						material: materials.add(handle.clone().into()),
						transform: Transform::from_xyz(0.0, 48.0, 0.0),
						..Default::default()
					});
				}
			}
			TileIndex::Animated(start, end, speed) => {
				// Do something  ✨ animated ✨
			}
		}
	}

	*has_ran = true;
}
