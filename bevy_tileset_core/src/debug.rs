//! Used for debugging tilesets

use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Local, Plugin, Transform};
use bevy::sprite::SpriteBundle;

use crate::prelude::{Tileset, Tilesets};

/// A component attached to the debug atlas sprite(s)
///
/// This can be used to query for the sprite(s) in other systems
#[derive(Component)]
pub struct DebugTilesetSprite;

/// A plugin used to debug tilesets, displaying them as sprites
#[derive(Default)]
pub struct DebugTilesetPlugin {
	/// The name of the tileset to display
	///
	/// If `None`, displays all tilesets arranged vertically
	pub tileset_name: Option<String>,
	/// The base position to display the sprite at
	///
	/// If `None`, displays at the world origin
	pub position: Vec3,
}

impl Plugin for DebugTilesetPlugin {
	fn build(&self, app: &mut App) {
		let state = DebugState {
			name: self.tileset_name.clone(),
			position: self.position,
		};
		app.add_system(display_tilesets(state));
	}
}

impl DebugTilesetPlugin {
	/// Displays the given tileset
	///
	/// # Arguments
	///
	/// * `tileset_name`: The name of the tileset
	///
	/// returns: DebugTilesetPlugin
	///
	pub fn single(tileset_name: &str) -> Self {
		Self {
			tileset_name: Some(tileset_name.to_string()),
			..Default::default()
		}
	}

	/// Displays the given tileset at a specified position
	///
	/// # Arguments
	///
	/// * `tileset_name`: The name of the tileset
	/// * `position`: The position to display at
	///
	/// returns: DebugTilesetPlugin
	///
	pub fn single_with_position(tileset_name: &str, position: Vec3) -> Self {
		Self {
			tileset_name: Some(tileset_name.to_string()),
			position,
		}
	}

	/// Displays all tilesets starting at a specified position
	///
	/// # Arguments
	///
	/// * `position`: The starting position to display at
	///
	/// returns: DebugTilesetPlugin
	///
	pub fn all(position: Vec3) -> Self {
		Self {
			tileset_name: None,
			position,
		}
	}
}

#[derive(Default)]
struct DebugState {
	name: Option<String>,
	position: Vec3,
}

fn display_tilesets(state: DebugState) -> impl FnMut(Local<bool>, Tilesets, Commands) {
	move |mut is_loaded: Local<bool>, tilesets: Tilesets, mut commands: Commands| {
		if *is_loaded {
			return;
		}

		let mut offset = Vec3::new(0.0, 0.0, 1.0);
		const PADDING: f32 = 10.0;

		let mut spawner = |tileset: &Tileset| {
			commands
				.spawn(SpriteBundle {
					texture: tileset.texture().clone(),
					transform: Transform::from_translation(state.position + offset),
					..Default::default()
				})
				.insert(DebugTilesetSprite);

			offset.y -= tileset.size().y + PADDING;
			*is_loaded = true;
		};

		if let Some(ref name) = state.name {
			// Specified tileset --> display single
			if let Some(tileset) = tilesets.get_by_name(name) {
				spawner(tileset);
			}
		} else {
			// No specified tileset --> display all
			for (.., tileset) in tilesets.iter() {
				spawner(tileset);
			}
		}
	}
}
