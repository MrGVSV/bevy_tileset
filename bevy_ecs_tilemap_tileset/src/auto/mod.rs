mod systems;
mod traits;

pub use systems::RemoveAutoTileEvent;
pub(crate) use systems::{on_change_auto_tile, on_remove_auto_tile};
