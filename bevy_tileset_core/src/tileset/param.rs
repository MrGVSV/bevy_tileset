use crate::prelude::{Tileset, TilesetId};
use bevy::asset::{Assets, Handle};
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, ResMut};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(SystemParam)]
pub struct Tilesets<'a> {
	tileset_map: ResMut<'a, TilesetMap>,
	tilesets: Res<'a, Assets<Tileset>>,
}

#[derive(Default)]
pub struct TilesetMap {
	name_to_id: HashMap<String, TilesetId>,
	id_to_handle: HashMap<TilesetId, Handle<Tileset>>,
	handle_to_id: HashMap<Handle<Tileset>, TilesetId>,
	id_to_name: HashMap<TilesetId, String>,
}

impl<'a> Deref for Tilesets<'a> {
	type Target = Res<'a, Assets<Tileset>>;

	fn deref(&self) -> &Self::Target {
		&self.tilesets
	}
}

impl<'a> Tilesets<'a> {
	/// Get a tileset by its ID.
	///
	/// # Arguments
	///
	/// * `id`: The tileset ID
	///
	/// returns: Option<&Tileset>
	pub fn get_by_id(&self, id: &TilesetId) -> Option<&Tileset> {
		let handle = self.tileset_map.id_to_handle.get(id)?;
		self.get(handle)
	}

	/// Get a tileset by its name
	///
	/// # Arguments
	///
	/// * `name`: The name of the tileset
	///
	/// returns: Option<&Tileset>
	pub fn get_by_name(&self, name: &str) -> Option<&Tileset> {
		let id = self.tileset_map.name_to_id.get(name)?;
		self.get_by_id(id)
	}

	/// Checks if a tileset with the given ID exists
	///
	/// # Arguments
	///
	/// * `id`: The tileset ID
	///
	/// returns: bool
	pub fn contains_id(&self, id: &TilesetId) -> bool {
		if let Some(handle) = self.tileset_map.id_to_handle.get(id) {
			// Check underlying asset to ensure the correct response is given
			self.contains(handle)
		} else {
			false
		}
	}

	/// Checks if a tileset with the given name exists
	///
	/// # Arguments
	///
	/// * `name`: The name of the tileset
	///
	/// returns: bool
	pub fn contains_name(&self, name: &str) -> bool {
		if let Some(id) = self.tileset_map.name_to_id.get(name) {
			// Check underlying asset to ensure the correct response is given
			self.contains_id(id)
		} else {
			false
		}
	}
}

impl TilesetMap {
	/// Register a tileset for easy lookup in the [Tilesets] system param.
	///
	/// # Arguments
	///
	/// * `tileset`: The tileset to register
	/// * `handle`: The handle to the tileset
	///
	/// returns: ()
	pub(crate) fn register_tileset(&mut self, tileset: &Tileset, handle: &Handle<Tileset>) {
		self.handle_to_id.insert(handle.clone_weak(), *tileset.id());
		self.id_to_name
			.insert(*tileset.id(), tileset.name().to_string());
		self.name_to_id
			.insert(tileset.name().to_string(), *tileset.id());
		self.id_to_handle.insert(*tileset.id(), handle.clone_weak());
	}

	/// Deregisters a tileset so it is no longer tracked
	///
	/// # Arguments
	///
	/// * `handle`: The handle to the tileset
	///
	/// returns: ()
	pub(crate) fn deregister_tileset(&mut self, handle: &Handle<Tileset>) {
		if let Some(ref id) = self.handle_to_id.remove(handle) {
			if let Some(ref name) = self.id_to_name.remove(id) {
				self.name_to_id.remove(name);
			}
			self.id_to_handle.remove(id);
		}
	}
}
