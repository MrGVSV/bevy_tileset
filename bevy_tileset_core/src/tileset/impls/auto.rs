//! Implementation details for Auto Tiles

use crate::prelude::{PartialTileId, RawTileset, TileIndex, Tileset};
use bevy_tileset_tiles::prelude::*;

macro_rules! impl_tileset {
	($name: ident) => {
		impl $name {
			/// Tries to get the [`TileIndex`] into the `TextureAtlas` for a tile with the given name,
			/// respecting rules defined by any auto tiles.
			///
			/// This method performs the same operations as [`get_tile_index`](crate::Tileset::get_tile_index), except that it also
			/// handles properly selecting tiles defined by [`TileType::Auto`].
			///
			/// # Arguments
			///
			/// * `name`: The name of the tile
			/// * `rule`: The rule to match
			///
			/// returns: Option<TileIndex>
			///
			/// # Examples
			///
			/// ```
			/// # use bevy::prelude::{Commands, Res};
			/// # use bevy_ecs_tilemap::MapQuery;
			/// # use bevy_tileset_core::prelude::*;
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
			/// 	let index = tileset.get_auto_index("My Auto Tile", rule);
			/// }
			/// ```
			pub fn get_auto_index(&self, name: &str, rule: AutoTileRule) -> Option<TileIndex> {
				let id = self.get_tile_group_id(name)?;
				self.get_auto_index_by_id(id, rule)
			}

			/// Like its counterpart [`get_auto_index`], this method attempts to get the [`TileIndex`] for a given tile.
			///
			/// This method, however, allows the specific auto tile to be chosen and/or its variant. This can be useful
			/// for reconstructing tiles with known indices.
			///
			/// If the ID has an `auto_index` of `None`, then the auto tile will be chosen based on the given rule.
			///
			/// # Arguments
			///
			/// * `id`: The ID of the tile
			/// * `rule`: The rule to match
			///
			/// returns: Option<TileIndex>
			///
			/// # Examples
			///
			/// ```
			/// # use bevy_tileset_core::prelude::*;
			/// fn get_index(tileset: &Tileset) {
			/// 	let index = tileset.get_auto_index_by_id(PartialTileId {
			/// 		group_id: 123,
			/// 		auto_index: Some(2),
			/// 		variant_index: None
			/// 	}, AutoTileRule::default());
			/// }
			/// ```
			pub fn get_auto_index_by_id<TId: Into<PartialTileId>>(
				&self,
				id: TId,
				rule: AutoTileRule,
			) -> Option<TileIndex> {
				let id = id.into();
				let group_id = id.group_id;
				let data = self.tiles.get(&group_id)?;

				match data.tile() {
					TileType::Auto(autos) => Self::select_auto(autos, rule, id),
					_ => self.get_tile_index_by_id(id),
				}
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

			pub(crate) fn select_auto<TId: Into<PartialTileId>>(
				auto_tiles: &[AutoTileData],
				rule: AutoTileRule,
				id: TId,
			) -> Option<TileIndex> {
				let id = id.into();
				let tile = if let Some(idx) = id.auto_index {
					auto_tiles.get(idx)?
				} else {
					match auto_tiles
						.iter()
						.find(|&auto| auto.rule().is_subset_of(&rule))
					{
						Some(t) => t,
						None => auto_tiles.last()?,
					}
				};

				let variant = if let Some(idx) = id.variant_index {
					println!("{}/{}", idx, tile.variants().len());
					tile.variants().get(idx)?
				} else {
					Self::select_variant(tile.variants())?
				};

				Some(variant.tile().into())
			}
		}
	};
}

impl_tileset!(Tileset);
impl_tileset!(RawTileset);
