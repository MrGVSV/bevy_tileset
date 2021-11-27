use crate::{Tileset, TilesetId};
use bevy::utils::HashMap;
use std::collections::hash_map::{Values, ValuesMut};

/// A resource containing all registered tilesets
#[derive(Default)]
pub struct Tilesets {
	ids: HashMap<String, TilesetId>,
	tilesets: HashMap<TilesetId, Tileset>,
	counter: TilesetId,
}

impl Tilesets {
	/// Get the ID of the tileset by name
	///
	/// # Arguments
	///
	/// * `name`: The tileset's name
	///
	/// returns: Option<&u8>
	///
	pub fn get_id(&self, name: &str) -> Option<&u8> {
		self.ids.get(name)
	}

	/// Get the tileset by ID
	///
	/// # Arguments
	///
	/// * `id`: The tileset's ID
	///
	/// returns: Option<&Tileset>
	///
	pub fn get(&self, id: &TilesetId) -> Option<&Tileset> {
		self.tilesets.get(id)
	}

	/// Get the tileset by name
	///
	/// # Arguments
	///
	/// * `name`: The tileset's name
	///
	/// returns: Option<&Tileset>
	///
	pub fn get_by_name(&self, name: &str) -> Option<&Tileset> {
		let id = self.get_id(name)?;
		self.tilesets.get(id)
	}

	/// Generate a new [`TilesetId`]
	///
	/// This should be attached to a tileset that's about to be registered
	pub fn next_id(&mut self) -> TilesetId {
		let id = self.counter;
		self.counter += 1u8;
		id
	}

	/// Register a new tileset
	///
	/// If the tileset replaces an existing one, the replaced tileset will be returned
	///
	/// # Arguments
	///
	/// * `tileset`: The tileset to register
	///
	/// returns: Option<Tileset>
	///
	pub fn register(&mut self, tileset: Tileset) -> Option<Tileset> {
		let id = tileset.id();
		self.ids.insert(tileset.name().to_string(), *id);
		self.tilesets.insert(*id, tileset)
	}

	/// Deregister a tileset by ID
	///
	/// # Arguments
	///
	/// * `id`: The tileset's ID
	///
	/// returns: Option<Tileset>
	///
	pub fn deregister(&mut self, id: &TilesetId) -> Option<Tileset> {
		self.tilesets.remove(id)
	}

	/// Deregister a tileset by name
	///
	/// # Arguments
	///
	/// * `name`: The tileset's name
	///
	/// returns: Option<Tileset>
	///
	pub fn deregister_by_name(&mut self, name: &str) -> Option<Tileset> {
		let id = self.ids.get(name)?;
		self.tilesets.remove(id)
	}

	/// Iterate over all registered tilesets
	pub fn iter(&self) -> Values<'_, TilesetId, Tileset> {
		self.tilesets.values()
	}

	/// Iterate mutably over all registered tilesets
	pub fn iter_mut(&mut self) -> ValuesMut<'_, TilesetId, Tileset> {
		self.tilesets.values_mut()
	}
}
