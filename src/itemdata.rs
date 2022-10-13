extern crate self as shrub; // this is needed so that derive(ItemData) works inside the package itself

use downcast_rs::{impl_downcast, Downcast};
use shrub_macros::ItemData;

pub trait ItemData: Downcast + 'static {}
impl_downcast!(ItemData);

#[derive(ItemData)]
pub struct EmptyItemData {}
