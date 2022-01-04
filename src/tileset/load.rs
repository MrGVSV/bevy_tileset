use bevy::asset::{Asset, AssetPath, AssetServer, Handle};
use bevy::prelude::Texture;
use bevy_tileset_tiles::prelude::*;

pub trait TextureLoader {
	fn load<'a, T: Asset, P: Into<AssetPath<'a>>>(&self, path: P) -> Handle<Texture>;
}

impl TextureLoader for AssetServer {
	fn load<'a, T: Asset, P: Into<AssetPath<'a>>>(&self, path: P) -> Handle<Texture> {
		self.load(path)
	}
}

pub fn load_tile_handles<TTiles: IntoIterator<Item = TileDef>, TLoader: TextureLoader>(
	tiles: TTiles,
	asset_loader: &TLoader,
) -> Vec<TileHandle> {
	tiles
		.into_iter()
		.map(|tile_def| TileHandle {
			name: tile_def.name.clone(),
			tile: match &tile_def.tile {
				TileDefType::Standard(path) => {
					TileHandleType::Standard(asset_loader.load::<Texture, &str>(path.as_str()))
				}
				TileDefType::Animated(anim) => {
					TileHandleType::Animated(load_animated(anim, asset_loader))
				}
				#[cfg(feature = "variants")]
				TileDefType::Variant(variants) => TileHandleType::Variant(
					variants
						.iter()
						.map(|variant| load_variant(variant, asset_loader))
						.collect(),
				),
				#[cfg(feature = "auto-tile")]
				TileDefType::Auto(autos) => TileHandleType::Auto(
					autos
						.iter()
						.map(|auto| load_auto(auto, asset_loader))
						.collect(),
				),
			},
		})
		.collect::<Vec<_>>()
}

fn load_animated<TLoader: TextureLoader>(
	def: &AnimatedTileDef,
	asset_loader: &TLoader,
) -> AnimatedTileHandle {
	AnimatedTileHandle {
		speed: def.speed,
		frames: def
			.frames
			.iter()
			.map(|frame| asset_loader.load::<Texture, &str>(frame.as_str()))
			.collect(),
	}
}

#[cfg(feature = "variants")]
fn load_variant<TLoader: TextureLoader>(
	def: &VariantTileDef,
	asset_loader: &TLoader,
) -> VariantTileHandle {
	VariantTileHandle {
		weight: def.weight,
		tile: match &def.tile {
			SimpleTileDefType::Standard(path) => {
				SimpleTileHandle::Standard(asset_loader.load::<Texture, &str>(path.as_str()))
			}
			SimpleTileDefType::Animated(anim) => {
				SimpleTileHandle::Animated(load_animated(anim, asset_loader))
			}
		},
	}
}

#[cfg(feature = "auto-tile")]
fn load_auto<TLoader: TextureLoader>(def: &AutoTileDef, asset_loader: &TLoader) -> AutoTileHandle {
	AutoTileHandle {
		rule: def.rule,
		variants: def
			.variants
			.iter()
			.map(|variant| load_variant(variant, asset_loader))
			.collect(),
	}
}
