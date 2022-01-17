use bevy::math::IVec2;

/// A tile's coordinates
///
/// At minimum, this should be able to return a tile's position in the tilemap. However, it may also contain
/// additional tile coordinate details such as layer index, chunk position, etc.
pub trait TileCoords {
	/// The tile position on the map
	///
	/// This is __not__ a tile's position in its chunk. It must be the actual integer coordinates of the tile's
	/// tilemap position.
	fn pos(&self) -> IVec2;
}
