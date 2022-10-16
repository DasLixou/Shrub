use std::{any::TypeId, collections::HashMap};

use crate::{item::ItemDataMap, itemdata_reflection::ItemDataReflection, Item, ItemData};

pub struct ItemType {
    data: ItemDataMap,
}

impl std::fmt::Debug for ItemType {
    // TODO: better impl for debug, maybe only print debug for itemdata which implement debug if possible
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItemType")
    }
}

impl ItemType {
    pub fn with_data<D: ItemDataReflection>(item_data: D) -> Self {
        let mut data = HashMap::with_capacity(D::CAPACITY);
        item_data.add_data(&mut data);
        ItemType { data }
    }

    pub fn with_capacity(data_capacity: usize) -> Self {
        let data = HashMap::with_capacity(data_capacity);
        ItemType { data }
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
            None => None,
        }
    }

    pub fn item_with_data<D: ItemDataReflection>(&self, item_data: D) -> Item {
        Item::with_data(self, item_data)
    }

    pub fn item_with_capacity(&self, data_capacity: usize) -> Item {
        Item::with_capacity(self, data_capacity)
    }
}

impl PartialEq for ItemType {
    fn eq(&self, other: &ItemType) -> bool {
        std::ptr::eq(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ItemData, ItemType};

    #[test]
    fn create_item() {
        let item_type = ItemType::with_capacity(0);
        let item = item_type.item_with_capacity(0);
        assert_eq!(item.item_type, &item_type);
    }

    #[test]
    fn create_item_with_capacity_data() {
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}

        let item_type = ItemType::with_capacity(0);
        let saturation = 8;
        let mut item = item_type.item_with_capacity(1);
        item.add_data(SaturationData { saturation });

        assert_eq!(item.item_type, &item_type);
        assert_eq!(
            item.get_data::<SaturationData>().unwrap().saturation,
            saturation
        );
    }

    #[test]
    fn create_item_with_multiple_data() {
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}
        struct SpeedData {
            speed: f32,
        }
        impl ItemData for SpeedData {}

        let item_type = ItemType::with_capacity(0);
        let saturation = 12;
        let speed = 13.4;
        let item = item_type.item_with_data((SaturationData { saturation }, SpeedData { speed }));
        assert_eq!(*item.item_type, item_type);
        assert_eq!(
            item.get_data::<SaturationData>().unwrap().saturation,
            saturation
        );
        assert_eq!(item.get_data::<SpeedData>().unwrap().speed, speed);
    }

    #[test]
    fn create_itemtype_with_data() {
        struct CoolData {
            is_cool: bool,
        }
        impl ItemData for CoolData {}

        let is_cool = true;
        let item_type = ItemType::with_data(CoolData { is_cool });
        assert_eq!(item_type.get_data::<CoolData>().unwrap().is_cool, is_cool);
        let item = item_type.item_with_capacity(0);
        assert_eq!(item.get_data::<CoolData>().unwrap().is_cool, is_cool);
    }

    #[test]
    fn create_itemtype_with_overriden_data() {
        struct NumberData {
            number: f32,
        }
        impl ItemData for NumberData {}

        let type_number = 12.5;
        let item_number = 74.12;

        let item_type = ItemType::with_data(NumberData {
            number: type_number,
        });
        let item = item_type.item_with_data(NumberData {
            number: item_number,
        });
        assert_eq!(
            item_type.get_data::<NumberData>().unwrap().number,
            type_number
        );
        assert_eq!(item.get_data::<NumberData>().unwrap().number, item_number);
    }
}
