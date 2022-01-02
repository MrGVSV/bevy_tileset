//! Contains tileset-related things

use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, Handle, Texture, UVec2, Vec2};
use bevy::reflect::TypeUuid;
use bevy::sprite::TextureAtlas;
use bevy_ecs_tilemap::{GPUAnimated, LayerBuilder, MapQuery, Tile, TileBundle};
use bevy_ecs_tileset_tiles::prelude::*;
use bevy_tile_atlas::{TextureStore, TileAtlasBuilder, TileAtlasBuilderError};
#[cfg(feature = "variants")]
use rand::{
	distributions::{Distribution, WeightedIndex},
	thread_rng,
};

use crate::prelude::*;
use crate::tileset::error::TilesetError;
use crate::PartialTileId;

/// A structure containing the registered tiles as well as their generated [`TextureAtlas`]
#[derive(Debug, TypeUuid)]
#[uuid = "4a176882-d7b2-429d-af5c-be418ccc3c52"]
pub struct Tileset {
	/// The ID of this tileset
	id: TilesetId,
	/// The name of this tileset
	name: String,
	/// The registered tiles mapped by their ID
	tiles: HashMap<TileGroupId, TileData>,
	/// The atlas for all registered tiles
	atlas: TextureAtlas,
	/// The size of the tiles in this tileset
	tile_size: Vec2,
	/// The tile group IDs mapped by their name
	tile_ids: HashMap<String, TileGroupId>,
	/// The tile names mapped by their ID
	tile_names: HashMap<TileGroupId, String>,
	/// The tile handles mapped by their index in the atlas
	tile_handles: HashMap<usize, Handle<Texture>>,
	/// The tile IDs mapped by their index in the atlas
	tile_indices: HashMap<usize, TileId>,
}

/// A builder for constructing a [`Tileset`]
#[derive(Default)]
pub struct TilesetBuilder {
	/// The registered tiles mapped by their ID
	tiles: HashMap<TileGroupId, TileData>,
	/// The builder used to construct the final [`TextureAtlas`]
	atlas_builder: TileAtlasBuilder,
	/// The tile IDs mapped by their name
	tile_ids: HashMap<String, TileGroupId>,
	/// The tile names mapped by their ID
	tile_names: HashMap<TileGroupId, String>,
	/// The tile handles mapped by their index in the atlas
	tile_handles: HashMap<usize, Handle<Texture>>,
	/// The tile IDs mapped by their index in the atlas
	tile_indices: HashMap<usize, PartialTileId>,
	/// The tile ID counter
	current_tile: TileCellId,
	/// The current tile group ID being processed
	current_group: TileGroupId,
}

impl Tileset {
	/// Gets the name of this tileset
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Gets the ID of this tileset
	pub fn id(&self) -> &TilesetId {
		&self.id
	}

	/// Gets the tileset [`TextureAtlas`]
	pub fn atlas(&self) -> &TextureAtlas {
		&self.atlas
	}

	/// Gets the handle to the [`TextureAtlas`]'s texture
	pub fn texture(&self) -> &Handle<Texture> {
		&self.atlas.texture
	}

	/// Gets the tile size for this tileset
	pub fn tile_size(&self) -> Vec2 {
		self.tile_size
	}

	/// Get the name of a tile by its ID
	///
	/// # Arguments
	///
	/// * `id`: The tile's ID
	///
	/// returns: Option<&String>
	///
	pub fn get_tile_name(&self, group_id: &TileGroupId) -> Option<&String> {
		self.tile_names.get(&group_id)
	}

	/// Get the base tile name for the given index
	///
	/// # Arguments
	///
	/// * `index`: The texture index
	///
	/// returns: Option<&String>
	///
	pub fn get_tile_name_by_index(&self, index: &usize) -> Option<&String> {
		let TileId { group_id, .. } = self.tile_indices.get(index)?;
		self.get_tile_name(group_id)
	}

	/// Get the group ID of a tile by its name
	///
	/// # Arguments
	///
	/// * `name`: The tile's name
	///
	/// returns: Option<&u32>
	///
	pub fn get_tile_group_id(&self, name: &str) -> Option<&TileGroupId> {
		self.tile_ids.get(name)
	}

	/// Get the ID of a tile by its index in the texture atlas
	///
	/// # Arguments
	///
	/// * `index`: The tile's index
	///
	/// returns: Option<&u32>
	///
	pub fn get_tile_id(&self, index: &usize) -> Option<&TileId> {
		self.tile_indices.get(index)
	}

	/// Get the handle of a tile by its index in the texture atlas
	///
	/// # Arguments
	///
	/// * `index`: The tile's index
	///
	/// returns: Option<&Handle<Texture>>
	///
	pub fn get_tile_handle(&self, index: &usize) -> Option<&Handle<Texture>> {
		self.tile_handles.get(index)
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
	/// # use bevy_ecs_tileset::TileIndex;
	///
	/// let index: TileIndex = tileset.get_tile_index("My Tile").unwrap();
	/// ```
	pub fn get_tile_index(&self, name: &str) -> Option<TileIndex> {
		let (index, ..) = self.get_tile_index_and_data(name)?;
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

	/// Get the data of a tile by its name
	///
	/// # Arguments
	///
	/// * `name`: The tile's name
	///
	/// returns: Option<&TileData>
	///
	pub fn get_tile_data(&self, name: &str) -> Option<&TileData> {
		let id = self.tile_ids.get(name)?;
		self.tiles.get(id)
	}

	/// Tries to get the [`TileIndex`] into the [`TextureAtlas`] for a tile with the given name,
	/// respecting rules defined by any auto tiles.
	///
	/// This method performs the same operations as [`get_tile_index`], except that it also
	/// handles properly selecting tiles defined by [`TileType::Auto`].
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
	/// # use bevy::prelude::{Commands, Res};
	/// # use bevy_ecs_tilemap::MapQuery;
	/// # use bevy_ecs_tileset::prelude::*;
	///
	/// fn place_tile(tileset: Res<Tileset>, mut commands: Commands, mut map_query: MapQuery) {
	/// 	// Matches:
	/// 	// - ✓ -
	/// 	// ✓ o ✓
	/// 	// - x -
	///    	let rule = AutoTileRule {
	///         north: Some(true),
	///         east: Some(true),
	///         west: Some(true),
	///         south: Some(false),
	///         ..Default::default()
	///     };
	///
	/// 	let index = tileset.get_auto_tile_index("My Auto Tile", rule);
	/// }
	/// ```
	#[cfg(feature = "auto-tile")]
	pub fn get_auto_tile_index(&self, name: &str, rule: AutoTileRule) -> Option<TileIndex> {
		let id = self.get_tile_group_id(name)?;
		let data = self.tiles.get(id)?;

		match data.tile() {
			TileType::Auto(autos) => Self::auto_index(autos, rule),
			_ => self.get_tile_index(name),
		}
	}

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
		let (tile_index, tile_data) = self.get_tile_index_and_data(name)?;

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
		let (tile_index, tile_data) = self.get_tile_index_and_data(name)?;

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
	pub fn update_entity(
		&self,
		name: &str,
		entity: Entity,
		commands: &mut Commands,
	) -> Option<Entity> {
		let (tile_index, tile_data) = self.get_tile_index_and_data(name)?;

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

	/// Randomly selects a variant from a collection of variants based on their weights
	///
	/// # Arguments
	///
	/// * `variants`: The variants to choose from
	///
	/// returns: Option<&VariantTileData>
	///
	#[cfg(feature = "variants")]
	pub fn select_variant(variants: &[VariantTileData]) -> Option<&VariantTileData> {
		let mut rng = thread_rng();
		let weights: Vec<f32> = variants.iter().map(|variant| variant.weight()).collect();
		let dist = WeightedIndex::new(weights).ok()?;
		let idx = dist.sample(&mut rng);
		variants.get(idx)
	}

	/// Checks if the given index is a variant for a given auto tile rule
	///
	/// This is an important method because it allows the auto tile system to skip tiles that
	/// already match a given rule.
	///
	/// Without this, for example, an auto tile with two variants may seem to swap between them
	/// when a neighbor requests that they check their state. The chosen auto tile hasn't changed,
	/// but the selected variant within that tile has. This method can be used to prevent something
	/// like this.
	///
	/// # Arguments
	///
	/// * `name`: The name of the auto tile
	/// * `index`: The texture index to check
	/// * `rule`: The rule that is a superset over the auto tile to match
	///
	/// returns: bool
	#[cfg(feature = "auto-tile")]
	pub fn is_auto_variant(&self, name: &str, index: &usize, rule: &AutoTileRule) -> bool {
		if let Some(data) = self.get_tile_data(name) {
			match data.tile() {
				TileType::Auto(autos) => {
					if let Some(auto) = autos.iter().find(|a| a.rule().is_subset_of(rule)) {
						// Check if _any_ variant matches the given index
						auto.variants()
							.iter()
							.any(|v| v.tile().contains_index(index))
					} else {
						false
					}
				}
				_ => false,
			}
		} else {
			false
		}
	}

	#[cfg(feature = "auto-tile")]
	fn auto_index(auto_tiles: &[AutoTileData], rule: AutoTileRule) -> Option<TileIndex> {
		let tile = match auto_tiles
			.iter()
			.find(|&auto| auto.rule().is_subset_of(&rule))
		{
			Some(t) => t,
			None => auto_tiles.last()?,
		};

		let variant = Self::select_variant(tile.variants())?;

		Some(variant.tile().into())
	}

	fn get_tile_index_and_data(&self, name: &str) -> Option<(TileIndex, &TileData)> {
		let id = self.get_tile_group_id(name)?;
		let data = self.tiles.get(id)?;

		Some((
			match data.tile() {
				TileType::Standard(index) => TileIndex::Standard(*index),
				TileType::Animated(anim) => {
					TileIndex::Animated(anim.start(), anim.end(), anim.speed())
				}
				#[cfg(feature = "variants")]
				TileType::Variant(variants) => {
					let variant = Self::select_variant(variants)?;
					variant.tile().into()
				}
				#[cfg(feature = "auto-tile")]
				TileType::Auto(autos) => Self::auto_index(autos, AutoTileRule::default())?,
			},
			data,
		))
	}
}

//    _____ _ _             _     ___      _ _    _
//   |_   _(_) |___ ___ ___| |_  | _ )_  _(_) |__| |___ _ _
//     | | | | / -_|_-</ -_)  _| | _ \ || | | / _` / -_) '_|
//     |_| |_|_\___/__/\___|\__| |___/\_,_|_|_\__,_\___|_|
//

impl TilesetBuilder {
	pub fn new(max_columns: Option<usize>) -> Self {
		let mut atlas_builder = TileAtlasBuilder::default();
		atlas_builder.max_columns(max_columns);
		Self {
			atlas_builder,
			tile_ids: Default::default(),
			current_group: Default::default(),
			current_tile: Default::default(),
			tile_indices: Default::default(),
			tile_names: Default::default(),
			tiles: Default::default(),
			tile_handles: Default::default(),
		}
	}

	/// Build the tileset
	///
	/// # Arguments
	///
	/// * `texture_store`: The texture assets
	///
	/// returns: Result<Tileset, TextureAtlasBuilderError>
	///
	pub fn build<TStore: TextureStore>(
		self,
		name: String,
		id: TilesetId,
		texture_store: &mut TStore,
	) -> Result<Tileset, TileAtlasBuilderError> {
		Ok(Tileset {
			name,
			id,
			tiles: self.tiles,
			tile_ids: self.tile_ids,
			tile_indices: self
				.tile_indices
				.into_iter()
				.map(|(key, value)| (key, value.extend(id)))
				.collect(),
			tile_names: self.tile_names,
			tile_handles: self.tile_handles,
			tile_size: self.atlas_builder.get_tile_size().unwrap_or_default(),
			atlas: self.atlas_builder.finish(texture_store)?,
		})
	}

	pub fn add_tile<TStore: TextureStore>(
		&mut self,
		tile_handle: TileHandle,
		group_id: TileGroupId,
		texture_store: &TStore,
	) -> Result<Option<TileData>, TilesetError> {
		let name = tile_handle.name.clone();

		self.current_group = group_id;
		self.current_tile = 0;

		let tile = TileData::new(
			tile_handle.name,
			self.get_tile_type(tile_handle.tile, texture_store)?,
		);

		self.tile_ids.insert(name.clone(), group_id);
		self.tile_names.insert(group_id, name);
		Ok(self.tiles.insert(group_id, tile))
	}

	fn get_tile_type<TStore: TextureStore>(
		&mut self,
		tile: TileHandleType,
		texture_store: &TStore,
	) -> Result<TileType, TilesetError> {
		Ok(match tile {
			TileHandleType::Standard(handle) => {
				TileType::Standard(self.insert_handle(&handle, texture_store)?)
			}
			TileHandleType::Animated(anim) => {
				TileType::Animated(self.create_animated(anim, texture_store)?)
			}
			#[cfg(feature = "variants")]
			TileHandleType::Variant(variants) => {
				TileType::Variant(self.create_variants(variants, texture_store)?)
			}
			#[cfg(feature = "auto-tile")]
			TileHandleType::Auto(autos) => TileType::Auto(self.create_autos(autos, texture_store)?),
		})
	}

	#[cfg(feature = "auto-tile")]
	fn create_autos<TStore: TextureStore>(
		&mut self,
		autos: Vec<AutoTileHandle>,
		texture_store: &TStore,
	) -> Result<Vec<AutoTileData>, TilesetError> {
		Ok(autos
			.into_iter()
			.map(|auto| -> Result<AutoTileData, TilesetError> {
				Ok(AutoTileData::new(
					auto.rule,
					self.create_variants(auto.variants, texture_store)?,
				))
			})
			.flat_map(|x| x.ok())
			.collect())
	}

	#[cfg(feature = "variants")]
	fn create_variants<TStore: TextureStore>(
		&mut self,
		variants: Vec<VariantTileHandle>,
		texture_store: &TStore,
	) -> Result<Vec<VariantTileData>, TilesetError> {
		Ok(variants
			.into_iter()
			.map(|variant| -> Result<VariantTileData, TilesetError> {
				Ok(VariantTileData::new(
					variant.weight,
					match variant.tile {
						SimpleTileHandle::Standard(handle) => {
							SimpleTileType::Standard(self.insert_handle(&handle, texture_store)?)
						}
						SimpleTileHandle::Animated(anim) => {
							SimpleTileType::Animated(self.create_animated(anim, texture_store)?)
						}
					},
				))
			})
			.filter_map(|x| x.ok())
			.collect())
	}

	fn create_animated<TStore: TextureStore>(
		&mut self,
		anim: AnimatedTileHandle,
		texture_store: &TStore,
	) -> Result<AnimatedTileData, TilesetError> {
		let (mut start, mut end) = (-1, -1);
		for frame in &anim.frames {
			let index = self.insert_handle(frame, texture_store)?;
			if start == -1 {
				start = index as i32;
			} else {
				end = index as i32;
			}
		}

		if start < 0 || end < 0 {
			return Err(TilesetError::InvalidData {
				expected: String::from("At least one animation frame"),
				found: String::from("Zero animation frames"),
			});
		}

		Ok(AnimatedTileData::new(
			anim.speed,
			start as usize,
			end as usize,
		))
	}

	fn insert_handle<TStore: TextureStore>(
		&mut self,
		handle: &Handle<Texture>,
		textures: &TStore,
	) -> Result<usize, TilesetError> {
		if let Some(texture) = textures.get(handle) {
			self.add_texture(handle, texture)
		} else {
			Err(TilesetError::ImageNotFound)
		}
	}

	pub fn add_texture(
		&mut self,
		handle: &Handle<Texture>,
		texture: &Texture,
	) -> Result<usize, TilesetError> {
		let index = self
			.atlas_builder
			.add_texture(handle.clone_weak(), texture)
			.map_err(|err| TilesetError::Atlas(err))?;

		self.tile_indices.insert(
			index,
			PartialTileId {
				group_id: self.current_group,
				cell_id: self.current_tile,
			},
		);
		self.tile_handles.insert(index, handle.clone_weak());

		self.current_tile += 1;

		Ok(index)
	}
}
