use crate::TileCoord;
use bevy::math::IVec2;
use bevy::prelude::{Entity, Query, With};
use bevy_ecs_tilemap::{MapQuery, Tile, TileParent, TilePos};
use bevy_tileset::auto::{AutoTile, AutoTileId};
use std::cell::RefCell;

/// A trait over `Query<'w, 's, (Entity, &TilePos, &TileParent, &AutoTile), With<Tile>>` to prevent errors with
/// "explicit lifetime required in the type of `query`"
pub(super) trait TileQuery {
	fn find_tile(&self, entity: Entity) -> Option<TileInfo>;
	fn count(&self) -> usize;
}

impl<'w, 's> TileQuery for Query<'w, 's, (Entity, &TilePos, &TileParent, &AutoTileId), With<Tile>> {
	fn find_tile(&self, entity: Entity) -> Option<TileInfo> {
		if let Ok((entity, pos, parent, auto_tile)) = self.get(entity) {
			Some(TileInfo::new(entity, pos, parent, auto_tile))
		} else {
			None
		}
	}

	fn count(&self) -> usize {
		self.iter().count()
	}
}

/// Defines a tile
#[derive(Copy, Clone, Debug)]
pub(super) struct TileInfo {
	pub coord: TileCoord,
	pub entity: Entity,
	pub auto_tile: bevy_tileset::auto::AutoTileId,
}

pub(super) struct TilemapCache<'a, 'w, 's> {
	pub tiles_query: &'a dyn TileQuery,
	pub map_query: &'a RefCell<MapQuery<'w, 's>>,
}

impl TileInfo {
	pub fn new(entity: Entity, pos: &TilePos, parent: &TileParent, auto_tile: &AutoTileId) -> Self {
		Self {
			entity,
			auto_tile: *auto_tile,
			coord: TileCoord {
				pos: *pos,
				map_id: parent.map_id,
				layer_id: parent.layer_id,
			},
		}
	}
}

impl bevy_tileset::auto::AutoTile for TileInfo {
	type Coords = TileCoord;

	fn coords(&self) -> Self::Coords {
		self.coord
	}

	fn auto_id(&self) -> bevy_tileset::auto::AutoTileId {
		self.auto_tile
	}

	fn can_match(&self, other: &Self) -> bool {
		self.coord.map_id == other.coord.map_id
			&& self.coord.layer_id == other.coord.layer_id
			&& self.auto_tile == other.auto_tile
	}
}

impl<'a, 'w, 's> bevy_tileset::auto::AutoTilemap for TilemapCache<'a, 'w, 's> {
	type Tile = TileInfo;

	fn make_coords(
		&self,
		pos: IVec2,
		template: &<Self::Tile as AutoTile>::Coords,
	) -> <Self::Tile as AutoTile>::Coords {
		TileCoord {
			pos: pos.as_uvec2().into(),
			map_id: template.map_id,
			layer_id: template.layer_id,
		}
	}

	fn get_tile_at(&self, coords: &<Self::Tile as AutoTile>::Coords) -> Option<Self::Tile> {
		let entity =
			self.map_query
				.borrow_mut()
				.get_tile_entity(coords.pos, coords.map_id, coords.layer_id);
		if let Ok(entity) = entity {
			self.tiles_query.find_tile(entity)
		} else {
			None
		}
	}

	fn len(&self) -> usize {
		self.tiles_query.count()
	}
}
