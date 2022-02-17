use crate::helpers::WorldCamera;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset_map::prelude::*;

pub fn build_map(
	tileset: &Tileset,
	map_size: MapSize,
	chunk_size: ChunkSize,
	layer_count: u16,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) -> Entity {
	let tileset_handle = tileset.texture().clone();

	let map_entity = commands.spawn().id();
	let mut map = Map::new(0u16, map_entity);

	let layers = build_layers(
		tileset,
		map_size,
		chunk_size,
		layer_count,
		&tileset_handle,
		commands,
		map_query,
	);

	for (index, layer) in layers.iter().enumerate() {
		let layer_id = index as u16;
		map.add_layer(commands, layer_id, *layer);
	}

	let tile_size = tileset.tile_size();
	let world_size = UVec2::new(map_size.0 * chunk_size.0, map_size.1 * chunk_size.1).as_vec2();
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
	map_size: MapSize,
	chunk_size: ChunkSize,
	layer_count: u16,
	material_handle: &Handle<Image>,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) -> Vec<Entity> {
	let texture_size = tileset.size();
	let settings = LayerSettings::new(
		map_size,
		chunk_size,
		tileset.tile_size().into(),
		TextureSize(texture_size.x, texture_size.y),
	);

	let mut layers = Vec::with_capacity(layer_count as usize);
	for layer_id in 0..layer_count {
		let settings = settings.clone();
		let material_handle = material_handle.clone();

		let layer_builder = LayerBuilder::<TileBundle>::new(commands, settings, 0u16, layer_id).0;

		let layer_entity = map_query.build_layer(commands, layer_builder, material_handle);
		layers.push(layer_entity);
	}

	layers
}
