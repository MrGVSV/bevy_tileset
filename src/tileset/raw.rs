use crate::prelude::{RawTileset, Tileset};
use bevy::prelude::Assets;
use bevy::sprite::TextureAtlas;

impl RawTileset {
	/// Converts this raw tileset into a finalized tileset asset
	pub fn into_asset(self, assets: &mut Assets<TextureAtlas>) -> Tileset {
		let texture = self.atlas().texture.clone();
		let atlas = assets.add(self.atlas);

		Tileset {
			id: self.id,
			name: self.name,
			tiles: self.tiles,
			size: self.size,
			tile_size: self.tile_size,
			tile_ids: self.tile_ids,
			tile_names: self.tile_names,
			tile_handles: self.tile_handles,
			tile_indices: self.tile_indices,
			atlas,
			texture,
		}
	}
}
