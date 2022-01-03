use crate::prelude::*;
use bevy::prelude::{Commands, Entity, UVec2};
use bevy_ecs_tilemap::{GPUAnimated, LayerBuilder, MapQuery, Tile, TileBundle};

impl Tileset {
	/// Initialize a tile
	///
	/// This should only be called when constructing the tilemap (hence the usage of `LayerBuilder`).
	/// Generally, the [`place_tile`] method is preferred.
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	/// * `position`: The position of the tile in the map
	/// * `commands`: The world `Commands`
	/// * `layer_builder`: The layer builder object
	///
	/// returns: Option<Entity>
	///
	/// # Examples
	///
	/// ```
	///	let entity = tileset.init_tile(
	/// 	"My Tile",
	/// 	UVec2::new(5, 5),
	/// 	&mut commands,
	/// 	&mut layer_builder
	/// ).unwrap();
	/// ```
	pub fn init_tile(
		&self,
		name: &str,
		position: UVec2,
		commands: &mut Commands,
		layer_builder: &mut LayerBuilder<TileBundle>,
	) -> Option<Entity> {
		let (tile_index, tile_data) = self.get_tile_tuple_by_name(name)?;

		let tile_entity = match tile_index {
			TileIndex::Standard(index) => {
				layer_builder
					.set_tile(
						position,
						Tile {
							texture_index: index as u16,
							..Default::default()
						}
						.into(),
					)
					.ok();
				layer_builder.get_tile_entity(position).ok()?
			}
			TileIndex::Animated(start, end, speed) => {
				layer_builder
					.set_tile(
						position,
						Tile {
							texture_index: start as u16,
							..Default::default()
						}
						.into(),
					)
					.ok();
				let entity = layer_builder.get_tile_entity(position).ok()?;
				commands
					.entity(entity)
					.insert(GPUAnimated::new(start as u32, end as u32, speed));
				entity
			}
		};

		#[cfg(feature = "auto-tile")]
		if let TileType::Auto(..) = tile_data.tile() {
			if let Some(tile_id) = self.get_tile_group_id(name) {
				commands
					.entity(tile_entity)
					.insert(AutoTile::new(*tile_id, self.id));
			}
		}

		Some(tile_entity)
	}

	/// Adds or sets a tile from the tileset
	///
	/// Automatically handles the tile's [`TileType`].
	///
	/// If successful, returns an optional tuple containing the old
	/// entity (if one existed) and the new entity.
	///
	/// # Arguments
	///
	/// * `name`:The name of the tile
	/// * `position`: The position of the tile in the map
	/// * `map_id`: The map ID
	/// * `layer_id`: The layer ID
	/// * `commands`: The world `Commands`
	/// * `map_query`: The tilemap query object
	///
	/// returns: Option<Entity>
	///
	/// # Examples
	///
	/// ```
	/// # use bevy::prelude::{Commands, Res};
	/// # use bevy_ecs_tilemap::prelude::MapQuery;
	/// # use bevy_ecs_tileset::prelude::Tileset;
	///
	/// fn place_tile(tileset: Res<Tileset>, mut commands: Commands, mut map_query: MapQuery) {
	///    	tileset.place_tile(
	/// 		"My Tile",
	/// 		UVec2::new(10, 10),
	/// 		0u16,
	/// 		0u16,
	/// 		&mut commands,
	/// 		&mut map_query
	/// 	);
	/// }
	/// ```
	pub fn place_tile(
		&self,
		name: &str,
		position: UVec2,
		map_id: u16,
		layer_id: u16,
		commands: &mut Commands,
		map_query: &mut MapQuery,
	) -> Option<(Option<Entity>, Entity)> {
		let old = map_query.get_tile_entity(position, map_id, layer_id).ok();
		let (tile_index, tile_data) = self.get_tile_tuple_by_name(name)?;

		let entity = match tile_index {
			TileIndex::Standard(index) => {
				let entity = map_query
					.set_tile(
						commands,
						position,
						Tile {
							texture_index: index as u16,
							..Default::default()
						},
						map_id,
						layer_id,
					)
					.ok()?;
				commands.entity(entity).remove::<GPUAnimated>().id()
			}
			TileIndex::Animated(start, end, speed) => {
				let entity = map_query
					.set_tile(
						commands,
						position,
						Tile {
							texture_index: start as u16,
							..Default::default()
						},
						map_id,
						layer_id,
					)
					.ok()?;
				commands
					.entity(entity)
					.insert(GPUAnimated::new(start as u32, end as u32, speed))
					.id()
			}
		};

		#[cfg(feature = "auto-tile")]
		{
			let mut cmds = commands.entity(entity);
			if let TileType::Auto(_) = tile_data.tile() {
				if let Some(tile_id) = self.get_tile_group_id(name) {
					cmds.insert(AutoTile::new(*tile_id, self.id));
				}
			} else {
				cmds.remove::<AutoTile>();
			}
		}

		map_query.notify_chunk_for_tile(position, map_id, layer_id);

		Some((old, entity))
	}

	/// Update an entity to match the given tile
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	/// * `entity`: The entity to update
	/// * `commands`: The world `Commands`
	///
	/// returns: Option<Entity>
	///
	pub fn update_tile(
		&self,
		name: &str,
		entity: Entity,
		commands: &mut Commands,
	) -> Option<Entity> {
		let (tile_index, tile_data) = self.get_tile_tuple_by_name(name)?;

		match tile_index {
			TileIndex::Standard(index) => {
				commands
					.entity(entity)
					.insert(Tile {
						texture_index: index as u16,
						..Default::default()
					})
					.remove::<GPUAnimated>();
			}
			TileIndex::Animated(start, end, speed) => {
				commands
					.entity(entity)
					.insert(Tile {
						texture_index: start as u16,
						..Default::default()
					})
					.insert(GPUAnimated::new(start as u32, end as u32 + 1u32, speed));
			}
		}

		#[cfg(feature = "auto-tile")]
		if let TileType::Auto(_) = tile_data.tile() {
			if let Some(tile_id) = self.get_tile_group_id(name) {
				commands
					.entity(entity)
					.insert(AutoTile::new(*tile_id, self.id));
			}
		} else {
			commands.entity(entity).remove::<AutoTile>();
		}

		Some(entity)
	}

	/// Tries to get the [`TileIndex`] into the [`TextureAtlas`] for a tile with the given name
	///
	/// Auto tiles are given a default rule and will return indices for whatever matches first. To
	/// get the correct indices for tiles defined as [`TileType::Auto`], the [`get_auto_tile_index`]
	/// should be used instead.
	///
	/// However, keep in mind that the auto tile system should automatically pick up an auto tile,
	/// assuming it has the [`AutoTile`] component attached to it.
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	///
	/// returns: Option<TileIndex>
	///
	/// # Examples
	///
	/// ```
	/// # use bevy_ecs_tileset::prelude::*;
	///
	/// let index: TileIndex = tileset.get_tile_index("My Tile").unwrap();
	/// ```
	pub fn get_tile_index(&self, name: &str) -> Option<TileIndex> {
		let (index, ..) = self.get_tile_tuple_by_name(name)?;
		Some(index)
	}

	pub fn get_tile_index_by_id<TId: Into<PartialTileId>>(&self, id: TId) -> Option<TileIndex> {
		let (index, ..) = self.get_tile_tuple_by_id(id)?;
		Some(index)
	}

	/// Tries to get the base index into the [`TextureAtlas`] for a tile with the given name
	///
	/// This is a convenience method around [`get_tile_index`] that performs the match expression
	/// returning the index if [`TileIndex::Standard`] or the start index if [`TileIndex::Animated`]
	///
	/// # Arguments
	///
	/// * `name`: The name of the tile
	///
	/// returns: Option<usize>
	///
	/// # Examples
	///
	/// ```
	/// let index: usize = tileset.get_base_tile_index("My Tile").unwrap();
	/// ```
	pub fn get_base_tile_index(&self, name: &str) -> Option<usize> {
		match self.get_tile_index(name)? {
			TileIndex::Standard(index) => Some(index),
			TileIndex::Animated(start, ..) => Some(start),
		}
	}

	pub(crate) fn get_tile_tuple_by_name(&self, name: &str) -> Option<(TileIndex, &TileData)> {
		let group_id = self.get_tile_group_id(name)?;
		self.get_tile_tuple_by_id(group_id)
	}

	pub(crate) fn get_tile_tuple_by_id<TId: Into<PartialTileId>>(
		&self,
		tile_id: TId,
	) -> Option<(TileIndex, &TileData)> {
		let id = tile_id.into();
		let group_id = id.group_id;
		let data = self.tiles.get(&group_id)?;

		Some((
			match data.tile() {
				TileType::Standard(index) => TileIndex::Standard(*index),
				TileType::Animated(anim) => {
					TileIndex::Animated(anim.start(), anim.end(), anim.speed())
				}
				#[cfg(feature = "variants")]
				TileType::Variant(variants) => {
					let variant = if let Some(idx) = id.variant_index {
						variants.get(idx)?
					} else {
						Self::select_variant(variants)?
					};
					variant.tile().into()
				}
				#[cfg(feature = "auto-tile")]
				TileType::Auto(autos) => Self::select_auto(autos, AutoTileRule::default(), id)?,
			},
			data,
		))
	}
}
