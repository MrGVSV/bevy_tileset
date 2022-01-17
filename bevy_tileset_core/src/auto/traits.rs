use crate::auto::AutoTileId;
use bevy::math::IVec2;
use bevy_tileset_tiles::auto::AutoTileRule;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

/// A struct containing a tile's data and Auto Tile rule
///
/// This is used to request that a tile be updated to match the given rule
pub struct AutoTileRequest<T: AutoTile> {
	pub tile: T,
	pub rule: AutoTileRule,
}

impl<T: AutoTile + Debug> Debug for AutoTileRequest<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TileUpdateRequest")
			.field("tile", &self.tile)
			.field("rule", &self.rule)
			.finish()
	}
}

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

/// A trait containing Auto Tile data
pub trait AutoTile {
	type Coords: TileCoords + Hash + Eq + Clone;

	/// Get the coordinates of this tile
	fn coords(&self) -> Self::Coords;
	/// Get the tile's current auto tile ID
	fn auto_id(&self) -> AutoTileId;
	/// Returns whether or not this tile can be matched against another
	///
	/// This is what allows auto tiles to be compared against one another. If, for example, you want tiles to
	/// only match within their layer, make sure you add a check ensuring that the two tiles are on the same layer.
	fn can_match(&self, other: &Self) -> bool;
	/// Get the tile's current position in the tilemap
	fn pos(&self) -> IVec2 {
		self.coords().pos()
	}
}

/// Provides methods of interacting with a tilemap, specifically for Auto Tiles
pub trait AutoTilemap {
	type Tile: AutoTile + Clone;

	/// Generate a new tile coordinates object
	///
	/// This method is also provided a "`template`" which is purely used to give additional details
	/// about how to generate the new position. This `template` assumes that its data is pertinent to the newly generated
	/// coordinates.
	///
	/// Essentially this allows things like layer index or chunk data to be passed around. But be caregul to verufy that the
	/// given `template` is actually valid for the requested position (i.e. it may come from a different layer or chunk than
	/// the one to be generated).
	///
	/// # Arguments
	///
	/// * `pos`: The new tilemap position to generate
	/// * `template`: A template coordinate object used to provide additonal context for the generation
	///
	/// returns: <Self::Tile as AutoTile>::Coords
	///
	/// # Examples
	///
	/// ```
	/// # use bevy::math::IVec2;
	/// # use bevy_tileset_core::auto::{AutoTile, AutoTilemap, TileCoords};
	///
	/// struct MyCoords{
	///   pos: IVec2,
	///   layer: u8
	/// }
	///
	/// impl TileCoords for MyCoords {
	///   fn pos(&self) -> IVec2 {
	///      self.pos
	///   }
	/// }
	///
	/// # struct MyTilemap;
	/// impl AutoTilemap for MyTilemap {
	///  // ...
	///  fn make_coords(&self, pos: IVec2, template: &<Self::Tile as AutoTile>::Coords) -> <Self::Tile as AutoTile>::Coords {
	///     MyCoords {
	///       pos,
	///       layer: template.layer
	///     }
	///   }
	///   // ...
	/// }
	///
	/// ```
	fn make_coords(
		&self,
		pos: IVec2,
		template: &<Self::Tile as AutoTile>::Coords,
	) -> <Self::Tile as AutoTile>::Coords;
	/// Get the tile at the given coordinates
	fn get_tile_at(&self, coords: &<Self::Tile as AutoTile>::Coords) -> Option<Self::Tile>;
	/// Get the number of Auto Tiles in this tilemap
	fn len(&self) -> usize;
}
