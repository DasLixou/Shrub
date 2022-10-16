use std::{any::TypeId, collections::HashMap};

use crate::{itemdata_reflection::ItemDataReflection, ItemData, ItemType};

pub(crate) type ItemDataMap = HashMap<TypeId, Box<dyn ItemData>>;

pub struct Item<'t> {
    pub item_type: &'t ItemType,
    data: ItemDataMap,
}

impl<'t> Item<'t> {
    pub(crate) fn with_data<D: ItemDataReflection>(item_type: &'t ItemType, item_data: D) -> Self {
        let mut data = HashMap::with_capacity(D::CAPACITY);
        item_data.add_data(&mut data);
        Item { item_type, data }
    }

    pub(crate) fn with_capacity(item_type: &'t ItemType, data_capacity: usize) -> Self {
        let data = HashMap::with_capacity(data_capacity);
        Item { item_type, data }
    }

    #[inline]
    pub fn add_data<D: ItemDataReflection>(&mut self, item_data: D) {
        self.data.reserve(D::CAPACITY);
        item_data.add_data(&mut self.data);
    }

    #[inline]
    pub fn get_data<D: ItemData>(&self) -> Option<&D> {
        match self.data.get(&TypeId::of::<D>()) {
            Some(d) => Some(d.as_any().downcast_ref::<D>().unwrap()),
            None => self.item_type.get_data::<D>(),
        }
    }

    /// # Attention
    /// Unlike `get_data`, `get_data_mut` can only borrow data from this item and will return `None` when the item doesnt include this Data and WON'T SEARCH IN THE ITEMTYPE for it.
    #[inline]
    pub fn get_data_mut<D: ItemData>(&mut self) -> Option<&mut D> {
        match self.data.get_mut(&TypeId::of::<D>()) {
            Some(d) => Some(d.as_any_mut().downcast_mut::<D>().unwrap()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Item, ItemData, ItemType};

    #[test]
    fn create_item_with_capacity() {
        let item_type = ItemType::with_capacity(0);
        let item = Item::with_capacity(&item_type, 0);
        assert_eq!(*item.item_type, item_type);
    }

    #[test]
    fn create_item_with_single_data() {
        let item_type = ItemType::with_capacity(0);
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}
        let saturation = 7;
        let item = Item::with_data(&item_type, SaturationData { saturation });
        assert_eq!(*item.item_type, item_type);
        assert_eq!(
            item.get_data::<SaturationData>().unwrap().saturation,
            saturation
        );
    }

    #[test]
    fn create_item_with_multiple_data() {
        let item_type = ItemType::with_capacity(0);
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}
        struct SpeedData {
            speed: f32,
        }
        impl ItemData for SpeedData {}
        let saturation = 7;
        let speed = 17.3;
        let item = Item::with_data(
            &item_type,
            (SaturationData { saturation }, SpeedData { speed }),
        );
        assert_eq!(*item.item_type, item_type);
        assert_eq!(
            item.get_data::<SaturationData>().unwrap().saturation,
            saturation
        );
        assert_eq!(item.get_data::<SpeedData>().unwrap().speed, speed);
    }

    #[test]
    fn create_item_with_single_mutable_data() {
        let item_type = ItemType::with_capacity(0);
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}
        let saturation = 7;
        let mut item = Item::with_data(&item_type, SaturationData { saturation });
        assert_eq!(*item.item_type, item_type);
        assert_eq!(
            item.get_data::<SaturationData>().unwrap().saturation,
            saturation
        );
        let new_sat = 12;
        item.get_data_mut::<SaturationData>().unwrap().saturation = new_sat;
        assert_eq!(
            item.get_data::<SaturationData>().unwrap().saturation,
            new_sat
        );
    }
}
