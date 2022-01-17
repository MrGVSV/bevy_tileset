use bevy_asset::Handle;
use bevy_render::texture::Image;
use serde::{Deserialize, Serialize};

/// A structure defining an animated tile
///
/// Made to be easily used with [`bevy_ecs_tilemap::GPUAnimated`] component
#[derive(Debug, Copy, Clone, Serialize)]
pub struct AnimatedTileData {
	/// The speed of the animation
	speed: f32,
	/// The start index of the animation (inclusive)
	start: usize,
	/// The end index of the animation (inclusive)
	end: usize,
}

/// A structure defining an animated tile
#[derive(Debug, Clone)]
pub struct AnimatedTileHandle {
	/// The speed of the animation
	pub speed: f32,
	/// The frames of the animation
	///
	/// Each frame is a registered [`Handle`]
	pub frames: Vec<Handle<Image>>,
}

/// A structure defining an animated tile
///
/// Made to be easily used with [`bevy_ecs_tilemap::GPUAnimated`] component
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnimatedTileDef {
	/// The speed of the animation
	///
	/// Default: 1.0
	#[serde(default = "default_speed")]
	pub speed: f32,
	/// The frames of the animation
	///
	/// Each entry is a path to a texture relative to the configuration file
	///
	/// # Examples
	///
	/// ```ron
	/// (
	/// 	// ...
	/// 	frames: [
	/// 		"frame-001.png",
	/// 		"frame-002.png",
	/// 		"frame-003.png",
	/// 	]
	/// 	// ...
	/// )
	/// ```
	#[serde(default)]
	pub frames: Vec<String>,
}

impl AnimatedTileData {
	pub fn new(speed: f32, start: usize, end: usize) -> Self {
		Self { speed, start, end }
	}

	/// Gets the start animation index (inclusive)
	pub fn start(&self) -> usize {
		self.start
	}

	/// Gets the end animation index (inclusive)
	pub fn end(&self) -> usize {
		self.end
	}

	/// Gets the animation speed
	pub fn speed(&self) -> f32 {
		self.speed
	}

	/// Gets the number of frames in this animation
	pub fn frame_count(&self) -> usize {
		self.end - self.start
	}
}

/// Gets the default animation speed
///
/// Used for deserialization
#[inline]
fn default_speed() -> f32 {
	1.0
}
