extern crate self as shrub; // this is needed so that derive(ItemData) works inside the package itself

use std::any::Any;

use shrub_macros::ItemData;

pub trait ItemData: 'static {
    // TODO: remove as_any because its weird, isnt there an other possibility?
    fn as_any(&self) -> &dyn Any;
}

#[derive(ItemData)]
pub struct EmptyItemData {}
