use crate::auto::traits::FindTile;
use crate::auto::AutoTile;
use bevy::math::IVec2;
use bevy::prelude::{Entity, Query, UVec2, With};
use bevy::utils::{HashMap, HashSet};
use bevy_ecs_tilemap::{MapQuery, Tile, TileParent};
use bevy_tileset::prelude::AutoTileRule;

/// The corrdinates of the tile, including the `map_id` and `layer_id`
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub(super) struct TileCoord {
	pub(super) pos: UVec2,
	pub(super) map_id: u16,
	pub(super) layer_id: u16,
}

/// Defines a tile
#[derive(Copy, Clone)]
pub(super) struct TileObject {
	pub(super) coord: TileCoord,
	pub(super) entity: Entity,
	pub(super) auto_tile: AutoTile,
}

/// An object containing the required data to update an auto tile
#[derive(Clone, Copy, Debug)]
pub(super) struct TileUpdateRequest {
	pub(super) entity: Entity,
	pub(super) rule: AutoTileRule,
}

/// A builder object that takes in auto tiles and calculates what changes need to be made
/// in accordance with their rules (including neighboring auto tiles as well)
pub(super) struct AutoTiler<'a, 'b> {
	tiles_query: &'a dyn FindTile,
	map_query: &'a MapQuery<'b>,
	cache: HashMap<TileCoord, TileObject>,
	requests: Vec<TileUpdateRequest>,
	requested: HashSet<TileCoord>,
}

impl TileObject {
	pub(super) fn new_with_parent(
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

	pub(super) fn new(entity: Entity, coord: TileCoord, auto_tile: &AutoTile) -> Self {
		TileObject {
			entity,
			coord,
			auto_tile: *auto_tile,
		}
	}
}

impl<'a, 'b> AutoTiler<'a, 'b> {
	pub(super) fn new(
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

	pub(super) fn requests(self) -> Vec<TileUpdateRequest> {
		self.requests
	}

	/// Processes the given tile and adds its generated requests to the list
	pub(super) fn add_tile(
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
