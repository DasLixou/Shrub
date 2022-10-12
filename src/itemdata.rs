// TODO: Maybe make data build up like an ECS

extern crate self as shrub; // this is needed so that derive(ItemData) works inside the package itself

use shrub_macros::ItemData;

pub trait ItemData {}

#[derive(ItemData)]
pub struct EmptyItemData {}
