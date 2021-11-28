//! Contains everything related to auto tiles

use std::fmt::Debug;
use std::hash::Hash;

use bevy::math::{IVec2, UVec2};
use bevy::prelude::{Changed, Commands, Entity, EventReader, Query, QuerySet, Res, With};

use crate::{TileId, TileIndex, TilesetId, Tilesets};
use bevy::utils::{HashMap, HashSet};
use bevy_ecs_tilemap::{GPUAnimated, MapQuery, Tile, TileParent};
use serde::{Deserialize, Serialize};

//     _____ _                   _
//    / ____| |                 | |
//   | (___ | |_ _ __ _   _  ___| |_ ___
//    \___ \| __| '__| | | |/ __| __/ __|
//    ____) | |_| |  | |_| | (__| |_\__ \
//   |_____/ \__|_|   \__,_|\___|\__|___/
//
//

/// A component used to ID a tile
///
/// Tiles with the same ID may enforce some type of automatic tiling
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AutoTile(TileId, TilesetId);

/// The rules used to define an auto tile
///
/// The possible states are:
/// * `Some(true)` -> Must Match
/// * `Some(false)` -> Must Not Match
/// * `None` -> Ignore
#[derive(Debug, Default, Deserialize, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct AutoTileRule {
	#[serde(alias = "n")]
	#[serde(default)]
	pub north: Option<bool>,
	#[serde(alias = "e")]
	#[serde(default)]
	pub east: Option<bool>,
	#[serde(alias = "s")]
	#[serde(default)]
	pub south: Option<bool>,
	#[serde(alias = "w")]
	#[serde(default)]
	pub west: Option<bool>,
	#[serde(alias = "ne")]
	#[serde(default)]
	pub north_east: Option<bool>,
	#[serde(alias = "nw")]
	#[serde(default)]
	pub north_west: Option<bool>,
	#[serde(alias = "se")]
	#[serde(default)]
	pub south_east: Option<bool>,
	#[serde(alias = "sw")]
	#[serde(default)]
	pub south_west: Option<bool>,
}

/// The corrdinates of the tile, including the `map_id` and `layer_id`
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
struct TileCoord {
	pos: UVec2,
	map_id: u16,
	layer_id: u16,
}

/// Defines a tile
#[derive(Copy, Clone)]
struct TileObject {
	coord: TileCoord,
	entity: Entity,
	auto_tile: AutoTile,
}

/// An object containing the required data to update an auto tile
#[derive(Clone, Copy, Debug)]
struct TileUpdateRequest {
	entity: Entity,
	rule: AutoTileRule,
}

/// A builder object that takes in auto tiles and calculates what changes need to be made
/// in accordance with their rules (including neighboring auto tiles as well)
struct AutoTiler<'a, 'b> {
	tiles_query: &'a dyn FindTile,
	map_query: &'a MapQuery<'b>,
	cache: HashMap<TileCoord, TileObject>,
	requests: Vec<TileUpdateRequest>,
	requested: HashSet<TileCoord>,
}

/// An event used to notify the system of a removed/replaced auto tile
pub struct RemoveAutoTileEvent(pub Entity);

//    _____                 _
//   |_   _|               | |
//     | |  _ __ ___  _ __ | |___
//     | | | '_ ` _ \| '_ \| / __|
//    _| |_| | | | | | |_) | \__ \
//   |_____|_| |_| |_| .__/|_|___/
//                   | |
//                   |_|

impl AutoTile {
	/// Create an `AutoTile`
	///
	/// # Arguments
	///
	/// * `id`: The ID used to identify tiles of this type
	/// * `tileset_id`: The ID of the associated tileset
	///
	/// returns: AutoTile
	///
	pub fn new(id: TileId, tileset_id: TilesetId) -> Self {
		Self(id, tileset_id)
	}

	/// Get the ID of this `AutoTile`
	pub fn id(&self) -> &TileId {
		&self.0
	}

	/// Get the ID of the associated [`Tileset`]
	pub fn tileset_id(&self) -> &TilesetId {
		&self.1
	}

	/// Set the ID of this AutoTile
	///
	/// Must match an existing auto tile in the associated [`Tileset`], otherwise it will have
	/// no effect on the auto tile system
	///
	/// # Arguments
	///
	/// * `id`: The new tile ID
	///
	/// returns: ()
	///
	pub fn set_id(&mut self, id: TileId) {
		self.0 = id;
	}

	/// Sets the ID for the associated tileset
	///
	/// Must match an existing [`Tileset`] in the [`Tilesets`] resource, otherwise it will have
	/// no effect on the auto tile system
	///
	/// # Arguments
	///
	/// * `tileset_id`: The associated tileset ID
	///
	/// returns: ()
	///
	pub fn set_tileset_id(&mut self, tileset_id: TilesetId) {
		self.1 = tileset_id;
	}
}

impl AutoTileRule {
	/// Checks if the given rule is a superset of this one.
	///
	/// > __ORDER MATTERS!!!__ This method checks if it itself is a subset of the given rule.
	/// Performing the opposite (i.e. swapping this rule with the given rule), may return a
	/// different value.
	///
	/// In our case, this rule, A, is a subset of B iff: A's rules perfectly match B's
	/// (i.e. `true == true` or `false == false`), except in cases where A's rule is defined
	/// as optional (i.e. `None`). So:
	///
	/// ```rust
	/// Some(true) ⊆ Some(true)
	/// Some(false) ⊆ Some(false)
	/// None ⊆ Some(true)
	/// None ⊆ Some(false)
	/// ```
	///
	/// <br/>
	///
	/// Note: if any direction returns false, the check short-circuits and returns false immediately,
	/// without checking the remaining directions.
	///
	/// # Arguments
	///
	/// * `other`: The other rule to check against
	///
	/// returns: bool
	///
	/// # Examples
	///
	/// ```
	/// use bevy_ecs_tilemap_tileset::prelude::*;
	/// let a = AutoTileRule { north: Some(true), ..Default::default() };
	/// let b = AutoTileRule { north: Some(true), east: Some(true), south: Some(false), ..Default::default() };
	///
	/// assert!(a.is_subset_of(&b)); // True since `b` contains `north: Some(true)`
	/// assert!(!b.is_subset_of(&a)); // False since `a` does not contain `east: Some(true)` nor `south: Some(false)`
	/// ```
	pub fn is_subset_of(&self, other: &AutoTileRule) -> bool {
		Self::check_bool(self.north, other.north)
			&& Self::check_bool(self.south, other.south)
			&& Self::check_bool(self.east, other.east)
			&& Self::check_bool(self.west, other.west)
			&& Self::check_bool(self.north_east, other.north_east)
			&& Self::check_bool(self.north_west, other.north_west)
			&& Self::check_bool(self.south_east, other.south_east)
			&& Self::check_bool(self.south_west, other.south_west)
	}

	/// Returns a default rule where all directions are set to `false`
	pub fn default_false() -> Self {
		Self {
			north: Some(false),
			east: Some(false),
			south: Some(false),
			west: Some(false),
			north_east: Some(false),
			north_west: Some(false),
			south_east: Some(false),
			south_west: Some(false),
		}
	}

	/// Returns a default rule where all directions are set to `true`
	pub fn default_true() -> Self {
		Self {
			north: Some(true),
			east: Some(true),
			south: Some(true),
			west: Some(true),
			north_east: Some(true),
			north_west: Some(true),
			south_east: Some(true),
			south_west: Some(true),
		}
	}

	fn check_bool(lhs: Option<bool>, rhs: Option<bool>) -> bool {
		match lhs {
			Some(l_val) => match rhs {
				Some(r_val) => l_val == r_val,
				None => !l_val,
			},
			None => true,
		}
	}
}

impl TileObject {
	fn new_with_parent(
		entity: Entity,
		pos: UVec2,
		parent: &TileParent,
		auto_tile: &AutoTile,
	) -> Self {
		TileObject {
			entity,
			coord: TileCoord {
				pos,
				map_id: parent.map_id,
				layer_id: parent.layer_id,
			},
			auto_tile: *auto_tile,
		}
	}

	fn new(entity: Entity, coord: TileCoord, auto_tile: &AutoTile) -> Self {
		TileObject {
			entity,
			coord,
			auto_tile: *auto_tile,
		}
	}
}

impl<'a, 'b> AutoTiler<'a, 'b> {
	fn new(
		tiles_query: &'a Query<(Entity, &UVec2, &TileParent, &AutoTile), With<Tile>>,
		map_query: &'a MapQuery<'b>,
	) -> Self {
		let total = tiles_query.iter().count();
		let capacity = total * 9usize;
		Self {
			tiles_query,
			map_query,
			cache: HashMap::with_capacity_and_hasher(capacity, Default::default()),
			requested: HashSet::with_capacity_and_hasher(capacity, Default::default()),
			requests: Vec::with_capacity(capacity),
		}
	}

	/// Processes the given tile and adds its generated requests to the list
	fn add_tile(
		&mut self,
		entity: Entity,
		pos: &UVec2,
		parent: &TileParent,
		auto_tile: &AutoTile,
		include_self: bool,
	) {
		let coord = TileCoord {
			pos: *pos,
			map_id: parent.map_id,
			layer_id: parent.layer_id,
		};

		if self.requested.contains(&coord) {
			// Tile has already been updated
			return;
		}

		let base_tile = TileObject::new(entity, coord, auto_tile);

		// Get all neighbors for self
		let neighbors = self.get_neighbors(base_tile.coord);
		// Filter for valid neighbors
		let neighbors = self.filter_neighbors(&base_tile, &neighbors);

		if include_self {
			let pos_i32 = base_tile.coord.pos.as_i32();
			let rule = self.generate_rule(&pos_i32, &neighbors);
			self.try_add_request(base_tile, rule);
		}

		// Update neighbors
		for neighbor in neighbors.into_iter() {
			let pos = neighbor.coord.pos;
			let pos_i32 = pos.as_i32();
			let sub_neighbors = self.get_neighbors(neighbor.coord);
			let sub_neighbors = self.filter_neighbors(&neighbor, &sub_neighbors);
			let rule = self.generate_rule(&pos_i32, &sub_neighbors);
			self.try_add_request(neighbor, rule);
		}
	}

	/// Filters surrounding tiles for valid "neighbors"
	/// (i.e. tiles on the same map and layer with a matching [`AutoTile`] component)
	fn filter_neighbors(
		&mut self,
		tile: &TileObject,
		neighbors: &[Option<(IVec2, Entity)>],
	) -> Vec<TileObject> {
		neighbors
			.iter()
			.filter_map(|n| *n)
			.map(|(pos, entity)| {
				let _pos_32 = pos.as_u32();
				let key = TileCoord {
					pos: pos.as_u32(),
					map_id: tile.coord.map_id,
					layer_id: tile.coord.layer_id,
				};
				if let Some(neighbor) = self.cache.get(&key) {
					if Self::is_valid_neighbor(tile, neighbor) {
						Some(*neighbor)
					} else {
						None
					}
				} else if let Some(neighbor) = self.tiles_query.find_tile(entity) {
					if Self::is_valid_neighbor(tile, &neighbor) {
						self.cache.insert(key, neighbor);
						Some(neighbor)
					} else {
						None
					}
				} else {
					None
				}
			})
			.flatten()
			.collect::<Vec<_>>()
	}

	/// Checks whether the given tile is a valid "neighbor" or not
	fn is_valid_neighbor(tile: &TileObject, neighbor: &TileObject) -> bool {
		tile.coord.map_id == neighbor.coord.map_id
			&& tile.coord.layer_id == neighbor.coord.layer_id
			&& tile.auto_tile == neighbor.auto_tile
	}

	/// Generate the rule for a given position based on the surrounding _valid_ neighbors
	fn generate_rule(&self, pos: &IVec2, neighbors: &[TileObject]) -> AutoTileRule {
		neighbors
			.iter()
			.fold(AutoTileRule::default(), |mut rule, neighbor| {
				let diff = neighbor.coord.pos.as_i32() - *pos;

				// === Northern === //
				if diff.y == 1i32 {
					if diff.x == 0i32 {
						rule.north = Some(true);
					} else if diff.x == -1i32 {
						rule.north_west = Some(true);
					} else {
						rule.north_east = Some(true);
					}
				}

				// === Parallel === //
				if diff.y == 0i32 {
					if diff.x == -1i32 {
						rule.west = Some(true);
					} else {
						rule.east = Some(true);
					}
				}

				// === Southern === //
				if diff.y == -1i32 {
					if diff.x == 0i32 {
						rule.south = Some(true);
					} else if diff.x == -1i32 {
						rule.south_west = Some(true);
					} else {
						rule.south_east = Some(true);
					}
				}

				rule
			})
	}

	/// Tries to add a request for the given tile
	///
	/// Tiles with missing/invalid indices in the [`Tileset`] will be skipped
	fn try_add_request(&mut self, tile: TileObject, rule: AutoTileRule) {
		let request = TileUpdateRequest {
			entity: tile.entity,
			rule,
		};

		self.requests.push(request);
		self.requested.insert(tile.coord);
	}

	/// Get the list of all surrounding tiles (whether valid neighbors or not)
	fn get_neighbors(&self, coord: TileCoord) -> [Option<(IVec2, Entity)>; 8] {
		[
			// === Northern === //
			self.get_neighbor_at_offset(-1, 1, coord),
			self.get_neighbor_at_offset(0, 1, coord),
			self.get_neighbor_at_offset(1, 1, coord),
			// === Parallel === //
			self.get_neighbor_at_offset(-1, 0, coord),
			self.get_neighbor_at_offset(1, 0, coord),
			// === Southern === //
			self.get_neighbor_at_offset(-1, -1, coord),
			self.get_neighbor_at_offset(0, -1, coord),
			self.get_neighbor_at_offset(1, -1, coord),
		]
	}

	/// get the neighbor at the given offset
	fn get_neighbor_at_offset(
		&self,
		offset_x: i32,
		offset_y: i32,
		coord: TileCoord,
	) -> Option<(IVec2, Entity)> {
		let offset = if offset_x != 0i32 || offset_y != 0i32 {
			IVec2::new(offset_x, offset_y)
		} else {
			// Skip self
			return None;
		};

		let n_pos = coord.pos.as_i32() + offset;
		let n_pos_32 = if n_pos.x >= 0i32 && n_pos.y >= 0i32 {
			n_pos.as_u32()
		} else {
			// Negative positions are not allowed (must convert to u32)
			return None;
		};

		let key = TileCoord {
			pos: n_pos_32,
			map_id: coord.map_id,
			layer_id: coord.layer_id,
		};

		if let Some(TileObject { entity, .. }) = self.cache.get(&key) {
			// Cached: Return the cached entity
			Some((n_pos, *entity))
		} else if let Ok(entity) =
			self.map_query
				.get_tile_entity(n_pos_32, coord.map_id, coord.layer_id)
		{
			// Not Cached: Locate the tile entity
			Some((n_pos, entity))
		} else {
			// No valid tile found
			None
		}
	}
}

//     _____           _
//    / ____|         | |
//   | (___  _   _ ___| |_ ___ _ __ ___  ___
//    \___ \| | | / __| __/ _ \ '_ ` _ \/ __|
//    ____) | |_| \__ \ ||  __/ | | | | \__ \
//   |_____/ \__, |___/\__\___|_| |_| |_|___/
//            __/ |
//           |___/

/// __\[SYSTEM\]__ Handles the creation/modification of an auto tile
///
/// This system chooses the appropriate texture based on its surrounding neighbors,
/// and updates any neighbors of the same type in a similar manner
pub(crate) fn on_change_auto_tile(
	mut commands: Commands,
	mut query: QuerySet<(
		// The newly added tile
		Query<(Entity, &UVec2, &TileParent, &AutoTile), (Changed<AutoTile>, With<Tile>)>,
		// All tiles
		Query<(Entity, &UVec2, &TileParent, &AutoTile), With<Tile>>,
		// Tiles to update
		Query<(
			Entity,
			&UVec2,
			&mut Tile,
			&TileParent,
			&AutoTile,
			Option<&mut GPUAnimated>,
		)>,
	)>,
	tilesets: Res<Tilesets>,
	mut map_query: MapQuery,
) {
	// Ensure a change happened
	if query.q0().iter().count() < 1 {
		return;
	}

	let mut tiler = AutoTiler::new(query.q1(), &map_query);

	for (entity, pos, parent, auto_tile) in query.q0().iter() {
		tiler.add_tile(entity, pos, parent, auto_tile, true);
	}

	let requests = tiler.requests;

	apply_requests(
		&requests,
		&tilesets,
		&mut query.q2_mut(),
		&mut commands,
		&mut map_query,
	);
}

/// __\[SYSTEM\]__ Handles the removal of auto tiles
///
/// Specifically, notifies the surrounding auto tiles of the change
/// This method needs to be called after the removal but within the same frame, otherwise
/// the query will be empty
pub(crate) fn on_remove_auto_tile(
	mut event: EventReader<RemoveAutoTileEvent>,
	mut query: QuerySet<(
		// All tiles
		Query<(Entity, &UVec2, &TileParent, &AutoTile), With<Tile>>,
		// Tiles to update
		Query<(
			Entity,
			&UVec2,
			&mut Tile,
			&TileParent,
			&AutoTile,
			Option<&mut GPUAnimated>,
		)>,
	)>,
	tilesets: Res<Tilesets>,
	mut map_query: MapQuery,
	mut commands: Commands,
) {
	let mut tiler = AutoTiler::new(query.q0(), &map_query);

	for RemoveAutoTileEvent(entity) in event.iter() {
		if let Ok((entity, pos, parent, auto_tile)) = query.q0().get(*entity) {
			tiler.add_tile(entity, pos, parent, auto_tile, false);
		}
	}

	let requests = tiler.requests;

	apply_requests(
		&requests,
		&tilesets,
		&mut query.q1_mut(),
		&mut commands,
		&mut map_query,
	);
}

/// Applies the given rule requests
fn apply_requests(
	requests: &[TileUpdateRequest],
	tilesets: &Tilesets,
	query: &mut Query<(
		Entity,
		&UVec2,
		&mut Tile,
		&TileParent,
		&AutoTile,
		Option<&mut GPUAnimated>,
	)>,
	commands: &mut Commands,
	map_query: &mut MapQuery,
) {
	for TileUpdateRequest { entity, rule } in requests.iter() {
		if let Ok((.., pos, ref mut tile, parent, auto_tile, ref mut anim)) = query.get_mut(*entity)
		{
			if let Some(tileset) = tilesets.get(auto_tile.tileset_id()) {
				if let Some(tile_name) = tileset.get_tile_name(auto_tile.id()) {
					// --- Check If Variant --- //
					let texture_index = tile.texture_index as usize;
					if tileset.is_auto_variant(tile_name, &texture_index, rule) {
						// The request index is just a variant of the correct state -> skip it
						continue;
					}

					// --- Apply Rule --- //
					if let Some(index) = tileset.get_auto_tile_index(tile_name, *rule) {
						match index {
							TileIndex::Standard(idx) => {
								tile.texture_index = idx as u16;

								// Remove animated if it exists
								if anim.is_some() {
									commands.entity(*entity).remove::<GPUAnimated>();
								}
							}
							TileIndex::Animated(start, end, speed) => {
								// Even though this texture index isn't seen (due to `GPUAnimated`), we still need to set this
								// so that the system can maintain the same variant across state changes
								tile.texture_index = start as u16;

								if let Some(anim) = anim {
									anim.start = start as u32;
									anim.end = end as u32;
									anim.speed = speed;
								} else {
									commands.entity(*entity).insert(GPUAnimated::new(
										start as u32,
										end as u32,
										speed,
									));
								}
							}
						}

						// --- Notify Chunk --- //
						map_query.notify_chunk_for_tile(*pos, parent.map_id, parent.layer_id);
					}
				}
			}
		}
	}
}

//    _______        _ _
//   |__   __|      (_) |
//      | |_ __ __ _ _| |_ ___
//      | | '__/ _` | | __/ __|
//      | | | | (_| | | |_\__ \
//      |_|_|  \__,_|_|\__|___/
//
//

trait FindTile {
	fn find_tile(&self, entity: Entity) -> Option<TileObject>;
}

impl<'w> FindTile for Query<'w, (Entity, &UVec2, &TileParent, &AutoTile), With<Tile>> {
	fn find_tile(&self, entity: Entity) -> Option<TileObject> {
		if let Ok((entity, pos, parent, auto_tile)) = self.get(entity) {
			Some(TileObject::new_with_parent(entity, *pos, parent, auto_tile))
		} else {
			None
		}
	}
}
