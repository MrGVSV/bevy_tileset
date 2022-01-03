use crate::prelude::Tileset;
use crate::tiles::prelude::*;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

impl Tileset {
	/// Randomly selects a variant from a collection of variants based on their weights
	///
	/// # Arguments
	///
	/// * `variants`: The variants to choose from
	///
	/// returns: Option<&VariantTileData>
	pub fn select_variant(variants: &[VariantTileData]) -> Option<&VariantTileData> {
		let mut rng = thread_rng();
		let weights: Vec<f32> = variants.iter().map(|variant| variant.weight()).collect();
		let dist = WeightedIndex::new(weights).ok()?;
		let idx = dist.sample(&mut rng);
		variants.get(idx)
	}
}
