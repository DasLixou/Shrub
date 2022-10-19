//! Shrub is an Item and Inventory System for games.
//!
//! # Features
//! - **proc** *(default)* &mdash; re-exports procedural macros from `shrub_macros`
//! - **serde** &mdash; derives `Serialize` and `Deserialize` traits
mod inventory;
mod item;
mod itemdata;
mod itemdata_reflection;
mod itemtype;

pub use inventory::Inventory;
pub use inventory::InventorySelector;
pub use item::Item;
pub use itemdata::ItemData;
pub use itemtype::ItemType;

#[cfg(feature = "proc")]
pub use shrub_macros::ItemData;
