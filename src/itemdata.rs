use downcast_rs::{impl_downcast, Downcast};

/// Declares that a struct can be used as data for an `Item` or `ItemType`
pub trait ItemData: Downcast + 'static {}
impl_downcast!(ItemData);
