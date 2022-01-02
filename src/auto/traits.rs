use crate::auto::auto_tiler::TileObject;
use crate::prelude::AutoTile;
use bevy::prelude::{Entity, Query, UVec2, With};
use bevy_ecs_tilemap::{Tile, TileParent};

pub(super) trait FindTile {
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
