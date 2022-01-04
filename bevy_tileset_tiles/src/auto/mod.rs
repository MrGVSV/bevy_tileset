mod rules;

use crate::prelude::{VariantTileData, VariantTileDef, VariantTileHandle};
pub use rules::AutoTileRule;
use serde::{Deserialize, Serialize};

/// A structure defining an auto tile
///
/// An auto tile contains rules that are applied when placed, removed, or changed
/// to itself and to its neighbors of the same type
#[derive(Debug, Clone, Serialize)]
pub struct AutoTileData {
	/// The rule defining this tile
	rule: AutoTileRule,
	/// The underlying tile variants
	variants: Vec<VariantTileData>,
}

/// A structure defining an auto tile
#[derive(Debug, Clone)]
pub struct AutoTileHandle {
	/// The rule defining this tile
	pub rule: AutoTileRule,
	/// The underlying variant handles
	pub variants: Vec<VariantTileHandle>,
}

/// A structure defining an auto tile
///
/// An auto tile contains rules that are applied when placed, removed, or changed
/// to itself and to its neighbors of the same type
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AutoTileDef {
	/// The rule defining this tile
	#[serde(default)]
	pub rule: AutoTileRule,
	/// The underlying tile variants
	#[serde(default)]
	pub variants: Vec<VariantTileDef>,
}

impl AutoTileData {
	pub fn new(rule: AutoTileRule, variants: Vec<VariantTileData>) -> Self {
		AutoTileData { rule, variants }
	}

	/// Gets the rule associated with this auto tile
	pub fn rule(&self) -> AutoTileRule {
		self.rule
	}

	/// Gets the underlying tile variants
	pub fn variants(&self) -> &Vec<VariantTileData> {
		&self.variants
	}
}
