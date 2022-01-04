use crate::helpers::WorldCamera;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap_tileset::prelude::*;

pub fn build_map(
	tileset: &Tileset,
	map_size: UVec2,
	chunk_size: UVec2,
	layer_count: u16,
	default_tile: u16,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) -> Entity {
	let tileset_handle = tileset.atlas().texture.clone();
	let tileset_material_handle = materials.add(tileset_handle.into());

	let map_entity = commands.spawn().id();
	let mut map = Map::new(0u16, map_entity);

	let layers = build_layers(
		tileset,
		map_size,
		chunk_size,
		layer_count,
		default_tile,
		&tileset_material_handle,
		commands,
		map_query,
	);

	for (index, layer) in layers.iter().enumerate() {
		let layer_id = index as u16;
		map.add_layer(commands, layer_id, *layer);
	}

	let tile_size = tileset.tile_size();
	let world_size = UVec2::new(map_size.x * chunk_size.x, map_size.y * chunk_size.y).as_f32();
	let mut offset = Vec2::new(tile_size.x * world_size.x, tile_size.y * world_size.y);
	offset /= 2.0;

	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(WorldCamera)
		.insert(Transform::from_xyz(offset.x, offset.y, 10.0));
	commands
		.entity(map_entity)
		.insert(map)
		.insert(Transform::from_xyz(0.0, 0.0, 0.0))
		.insert(GlobalTransform::default());

	map_entity
}

pub fn build_layers(
	tileset: &Tileset,
	map_size: UVec2,
	chunk_size: UVec2,
	layer_count: u16,
	default_tile: u16,
	material_handle: &Handle<ColorMaterial>,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) -> Vec<Entity> {
	let settings = LayerSettings::new(
		map_size,
		chunk_size,
		tileset.tile_size(),
		tileset.atlas().size,
	);

	let mut layers = Vec::with_capacity(layer_count as usize);
	for layer_id in 0..layer_count {
		let settings = settings.clone();
		let material_handle = material_handle.clone();

		let (mut layer_builder, _) = LayerBuilder::new(commands, settings, 0u16, layer_id);
		layer_builder.set_all(TileBundle {
			tile: Tile {
				texture_index: default_tile,
				..Default::default()
			},
			..Default::default()
		});

		let layer_entity = map_query.build_layer(commands, layer_builder, material_handle);
		layers.push(layer_entity);
	}

	layers
}
