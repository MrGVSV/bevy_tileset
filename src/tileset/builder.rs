use crate::ids::PartialTileId;
use crate::prelude::*;
use crate::tiles::prelude::*;
use bevy::prelude::{Handle, Texture};
use bevy_tile_atlas::{TextureStore, TileAtlasBuilder, TileAtlasBuilderError};
use std::collections::HashMap;

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
	/// The current tile group ID being processed
	current_group: TileGroupId,
}

impl TilesetBuilder {
	pub fn new(max_columns: Option<usize>) -> Self {
		let mut atlas_builder = TileAtlasBuilder::default();
		atlas_builder.max_columns(max_columns);
		Self {
			atlas_builder,
			tile_ids: Default::default(),
			current_group: Default::default(),
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
	/// * `texture_store`: The store of textures
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

	/// Add a tile to the tileset being built
	///
	/// # Arguments
	///
	/// * `tile_handle`: The tile to add
	/// * `group_id`: The group ID of the tile (this should be unique across tiles)
	/// * `texture_store`: The store of textures
	///
	/// returns: Result<Option<TileData>, TilesetError>
	///
	/// # Examples
	///
	/// ```
	/// # use bevy::prelude::*;
	/// # use bevy_ecs_tileset::prelude::*;
	/// # use bevy_ecs_tileset::tiles::*;
	///
	/// fn tileset_creator(textures: Res<Assets<Texture>>) {
	/// 	let mut builder = TilesetBuilder::default();
	/// 	let tile = TileHandle::new_standard("My Tile", Handle::default());
	/// 	builder.add_tile(tile, 123, &textures);
	/// 	// ...
	/// }
	/// ```
	pub fn add_tile<TStore: TextureStore>(
		&mut self,
		tile_handle: TileHandle,
		group_id: TileGroupId,
		texture_store: &TStore,
	) -> Result<Option<TileData>, TilesetError> {
		if self.tiles.contains_key(&group_id) {
			return Err(TilesetError::TileAlreadyExists(group_id));
		}

		let name = tile_handle.name.clone();

		self.current_group = group_id;

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

		let id = PartialTileId::new(self.current_group);
		self.tile_indices.insert(index, id);
		self.tile_handles.insert(index, handle.clone_weak());

		Ok(index)
	}
}
