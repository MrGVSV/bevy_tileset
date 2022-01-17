use serde::{Deserialize, Serialize};

/// The rules used to define an auto tile
///
/// The possible states are:
/// * `Some(true)` -> Must Match
/// * `Some(false)` -> Must Not Match
/// * `None` -> Ignore
#[derive(Debug, Default, Deserialize, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct AutoTileRule {
	#[serde(alias = "n")]
	#[serde(default)]
	pub north: Option<bool>,
	#[serde(alias = "e")]
	#[serde(default)]
	pub east: Option<bool>,
	#[serde(alias = "s")]
	#[serde(default)]
	pub south: Option<bool>,
	#[serde(alias = "w")]
	#[serde(default)]
	pub west: Option<bool>,
	#[serde(alias = "ne")]
	#[serde(default)]
	pub north_east: Option<bool>,
	#[serde(alias = "nw")]
	#[serde(default)]
	pub north_west: Option<bool>,
	#[serde(alias = "se")]
	#[serde(default)]
	pub south_east: Option<bool>,
	#[serde(alias = "sw")]
	#[serde(default)]
	pub south_west: Option<bool>,
}

impl AutoTileRule {
	/// Checks if the given rule is a superset of this one.
	///
	/// > __ORDER MATTERS!!!__ This method checks if it itself is a subset of the given rule.
	/// Performing the opposite (i.e. swapping this rule with the given rule), may return a
	/// different value.
	///
	/// In our case, this rule, A, is a subset of B iff: A's rules perfectly match B's
	/// (i.e. `true == true` or `false == false`), except in cases where A's rule is defined
	/// as optional (i.e. `None`). So:
	///
	/// * `Some(true)` ⊆ `Some(true)`
	/// * `Some(false)` ⊆ `Some(false)`
	/// * `None` ⊆ `Some(true)`
	/// * `None` ⊆ `Some(false)`
	///
	///
	/// Note: if any direction returns false, the check short-circuits and returns false immediately,
	/// without checking the remaining directions.
	///
	/// # Arguments
	///
	/// * `other`: The other rule to check against
	///
	/// returns: bool
	///
	/// # Examples
	///
	/// ```
	/// # use bevy_tileset_tiles::prelude::AutoTileRule;
	///
	/// let a = AutoTileRule { north: Some(true), ..Default::default() };
	/// let b = AutoTileRule { north: Some(true), east: Some(true), south: Some(false), ..Default::default() };
	///
	/// assert!(a.is_subset_of(&b)); // True since `b` contains `north: Some(true)`
	/// assert!(!b.is_subset_of(&a)); // False since `a` does not contain `east: Some(true)` nor `south: Some(false)`
	/// ```
	pub fn is_subset_of(&self, other: &AutoTileRule) -> bool {
		Self::check_bool(self.north, other.north)
			&& Self::check_bool(self.south, other.south)
			&& Self::check_bool(self.east, other.east)
			&& Self::check_bool(self.west, other.west)
			&& Self::check_bool(self.north_east, other.north_east)
			&& Self::check_bool(self.north_west, other.north_west)
			&& Self::check_bool(self.south_east, other.south_east)
			&& Self::check_bool(self.south_west, other.south_west)
	}

	/// Returns a default rule where all directions are set to `false`
	pub fn default_false() -> Self {
		Self {
			north: Some(false),
			east: Some(false),
			south: Some(false),
			west: Some(false),
			north_east: Some(false),
			north_west: Some(false),
			south_east: Some(false),
			south_west: Some(false),
		}
	}

	/// Returns a default rule where all directions are set to `true`
	pub fn default_true() -> Self {
		Self {
			north: Some(true),
			east: Some(true),
			south: Some(true),
			west: Some(true),
			north_east: Some(true),
			north_west: Some(true),
			south_east: Some(true),
			south_west: Some(true),
		}
	}

	fn check_bool(lhs: Option<bool>, rhs: Option<bool>) -> bool {
		match lhs {
			Some(l_val) => match rhs {
				Some(r_val) => l_val == r_val,
				None => !l_val,
			},
			None => true,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::AutoTileRule;

	#[test]
	fn should_be_subset() {
		let a = AutoTileRule {
			north: Some(true),
			..Default::default()
		};
		let b = AutoTileRule {
			north: Some(true),
			east: Some(true),
			south: Some(false),
			..Default::default()
		};

		assert!(a.is_subset_of(&b));
		assert!(!b.is_subset_of(&a));
	}
}
