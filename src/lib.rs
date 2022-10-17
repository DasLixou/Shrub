//! Shrub is an Item and Inventory System for games.
//!
//! # Features
//! - **proc** *(default)* &mdash; re-exports procedural macros from `shrub_macros`
mod inventory;
mod item;
mod itemdata;
mod itemdata_reflection;
mod itemtype;
mod registry;

pub use inventory::Inventory;
pub use item::Item;
pub use itemdata::ItemData;
pub use itemtype::ItemType;
pub use registry::Registry;

#[cfg(feature = "proc")]
pub use shrub_macros::ItemData;
