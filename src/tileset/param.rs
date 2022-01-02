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
	pub fn get_by_id(&self, id: &TilesetId) -> Option<&Tileset> {
		let handle = self.tileset_map.id_to_handle.get(id)?;
		self.get(handle)
	}

	pub fn get_by_name(&self, name: &str) -> Option<&Tileset> {
		let id = self.tileset_map.name_to_id.get(name)?;
		self.get_by_id(id)
	}
}

impl TilesetMap {
	pub fn add_tileset(&mut self, tileset: &Tileset, handle: &Handle<Tileset>) {
		self.handle_to_id.insert(handle.clone_weak(), *tileset.id());
		self.id_to_name
			.insert(*tileset.id(), tileset.name().to_string());
		self.name_to_id
			.insert(tileset.name().to_string(), *tileset.id());
		self.id_to_handle.insert(*tileset.id(), handle.clone_weak());
	}

	pub fn remove_tileset(&mut self, handle: &Handle<Tileset>) {
		if let Some(ref id) = self.handle_to_id.remove(handle) {
			if let Some(ref name) = self.id_to_name.remove(id) {
				self.name_to_id.remove(name);
			}
			self.id_to_handle.remove(id);
		}
	}
}
