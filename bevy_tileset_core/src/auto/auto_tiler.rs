use crate::auto::traits::{AutoTile, AutoTileRequest, AutoTilemap};
use crate::coords::TileCoords;
use bevy::math::IVec2;
use bevy::utils::{HashMap, HashSet};
use bevy_tileset_tiles::auto::AutoTileRule;

/// A builder object that takes in auto tiles and calculates what changes need to be made
/// in accordance with their rules.
///
/// The returned [`AutoTileRequest`] collection includes how to update the added tiles and their
/// neighbors.
pub struct AutoTiler<'a, T: AutoTilemap> {
	tilemap: &'a mut T,
	cache: HashMap<<T::Tile as AutoTile>::Coords, T::Tile>,
	requests: Vec<AutoTileRequest<T::Tile>>,
	requested: HashSet<<T::Tile as AutoTile>::Coords>,
}

impl<'a, T: AutoTilemap> AutoTiler<'a, T> {
	pub fn new(tilemap: &'a mut T) -> Self {
		let total = tilemap.len();
		// Each tile added has the potential to create 9 requests: itself and 8 neighbors
		let capacity = total * 9usize;
		Self {
			tilemap,
			cache: HashMap::with_capacity_and_hasher(capacity, Default::default()),
			requested: HashSet::with_capacity_and_hasher(capacity, Default::default()),
			requests: Vec::with_capacity(capacity),
		}
	}

	/// Finish generating the auto tile requests and return them
	pub fn finish(self) -> Vec<AutoTileRequest<T::Tile>> {
		self.requests
	}

	/// Processes the given tile and its neighbors (if needed), adding any generated requests to the
	/// current collection.
	///
	/// # Arguments
	///
	/// * `tile`: The tile to add
	/// * `include_self`: If false, only the given tile's neighbors will be processed. This is useful for
	///                   handling removals.
	///
	/// returns: ()
	pub fn add_tile(&mut self, tile: T::Tile, include_self: bool) {
		let coords = tile.coords();

		if self.requested.contains(&coords) {
			// Tile has already been updated
			return;
		}

		// Get all neighbors for self
		let neighbors = self.get_neighbors(&tile);
		// Filter for valid neighbors
		let neighbors = self.filter_neighbors(&tile, &neighbors);

		if include_self {
			let pos_i32 = tile.pos();
			let rule = self.generate_rule(&pos_i32, &neighbors);
			self.try_add_request(tile, rule);
		}

		// Update neighbors
		for neighbor in neighbors.into_iter() {
			let pos = neighbor.pos();
			let sub_neighbors = self.get_neighbors(&neighbor);
			let sub_neighbors = self.filter_neighbors(&neighbor, &sub_neighbors);
			let rule = self.generate_rule(&pos, &sub_neighbors);
			self.try_add_request(neighbor, rule);
		}
	}

	/// Tries to add a request for the given tile
	fn try_add_request(&mut self, tile: T::Tile, rule: AutoTileRule) {
		self.requested.insert(tile.coords());
		let request = AutoTileRequest { tile, rule };
		self.requests.push(request);
	}

	// region Neighbors

	/// Get the list of all surrounding tiles (whether valid neighbors or not)
	fn get_neighbors(&self, tile: &T::Tile) -> [Option<T::Tile>; 8] {
		let coords = tile.coords();
		[
			// === Northern === //
			self.get_neighbor_at_offset(-1, 1, &coords),
			self.get_neighbor_at_offset(0, 1, &coords),
			self.get_neighbor_at_offset(1, 1, &coords),
			// === Parallel === //
			self.get_neighbor_at_offset(-1, 0, &coords),
			self.get_neighbor_at_offset(1, 0, &coords),
			// === Southern === //
			self.get_neighbor_at_offset(-1, -1, &coords),
			self.get_neighbor_at_offset(0, -1, &coords),
			self.get_neighbor_at_offset(1, -1, &coords),
		]
	}

	/// Get the neighbor at the given offset
	fn get_neighbor_at_offset(
		&self,
		offset_x: i32,
		offset_y: i32,
		coords: &<T::Tile as AutoTile>::Coords,
	) -> Option<T::Tile> {
		let offset = if offset_x != 0i32 || offset_y != 0i32 {
			IVec2::new(offset_x, offset_y)
		} else {
			// Skip self
			return None;
		};

		let n_pos: IVec2 = coords.pos() + offset;
		let n_coords = self.tilemap.make_coords(n_pos, coords);

		if let Some(tile) = self.cache.get(&n_coords) {
			// Cached: Return the cached entity
			Some(tile.clone())
		} else if let Some(tile) = self.tilemap.get_tile_at(&n_coords) {
			// Not Cached: Locate the tile entity
			Some(tile.clone())
		} else {
			// No valid tile found
			None
		}
	}

	/// Filters surrounding tiles for valid "neighbors"
	/// (i.e. tiles on the same map and layer with a matching [`AutoTile`] component)
	fn filter_neighbors(&mut self, tile: &T::Tile, neighbors: &[Option<T::Tile>]) -> Vec<T::Tile> {
		neighbors
			.iter()
			.filter(|n| n.is_some())
			.map(|n| n.as_ref().unwrap())
			.map(|neighbor| {
				let n_coords = neighbor.coords();
				if let Some(neighbor) = self.cache.get(&n_coords) {
					if tile.can_match(neighbor) {
						Some(neighbor.clone())
					} else {
						None
					}
				} else if let Some(neighbor) = self.tilemap.get_tile_at(&n_coords) {
					if tile.can_match(&neighbor) {
						self.cache.insert(n_coords, neighbor.clone());
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
	// endregion

	/// Generate the rule for a given position based on the surrounding _valid_ neighbors
	fn generate_rule(&self, pos: &IVec2, neighbors: &[T::Tile]) -> AutoTileRule {
		neighbors
			.iter()
			.fold(AutoTileRule::default(), |mut rule, neighbor| {
				let diff = neighbor.pos() - *pos;

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
}
