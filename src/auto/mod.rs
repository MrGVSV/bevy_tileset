use crate::prelude::{TileGroupId, TilesetId};

mod auto_tiler;
mod systems;
mod traits;

pub use systems::RemoveAutoTileEvent;
pub(crate) use systems::{on_change_auto_tile, on_remove_auto_tile};

/// A component used to ID a tile
///
/// Tiles with the same ID may enforce some type of automatic tiling
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AutoTile(TileGroupId, TilesetId);

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
	pub fn new(id: TileGroupId, tileset_id: TilesetId) -> Self {
		Self(id, tileset_id)
	}

	/// Get the ID of this `AutoTile`
	pub fn id(&self) -> &TileGroupId {
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
	pub fn set_id(&mut self, id: TileGroupId) {
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
