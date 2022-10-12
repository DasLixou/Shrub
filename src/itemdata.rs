// TODO: Maybe make data build up like an ECS

pub trait ItemData {}

// #[derive(ItemData)] // TODO: Make derive work for ItemData, since its only a marker
pub struct EmptyItemData {}

impl ItemData for EmptyItemData {}
