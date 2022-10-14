use std::{any::TypeId, collections::HashMap};

use crate::{utils::add_data::ItemDataHack, ItemData, ItemType};

pub struct Item<'t> {
    pub item_type: &'t ItemType,
    data: HashMap<TypeId, Box<dyn ItemData>>,
}

impl<'t> Item<'t> {
    pub(crate) fn with_data<D: ItemDataHack>(item_type: &'t ItemType, item_data: D) -> Self {
        let mut data = HashMap::with_capacity(D::CAPACITY);
        item_data.add_data(&mut data);
        Item { item_type, data }
    }

    pub(crate) fn with_capacity(item_type: &'t ItemType, data_capacity: usize) -> Self {
        let data = HashMap::with_capacity(data_capacity);
        Item { item_type, data }
    }

    #[inline]
    pub fn add_data<D: ItemData>(&mut self, data: D) {
        self.data.insert(TypeId::of::<D>(), Box::new(data));
    }

    #[inline]
    pub fn get_data<D: ItemData>(&self) -> &D {
        let data = self.data.get(&TypeId::of::<D>()).unwrap();
        // TODO: unwrap check
        data.as_any().downcast_ref::<D>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Item, ItemData, ItemType};

    #[test]
    fn create_item_with_capacity() {
        let item_type = ItemType {};
        let item = Item::with_capacity(&item_type, 0);
        assert_eq!(*item.item_type, item_type);
    }

    #[test]
    fn create_item_with_single_data() {
        let item_type = ItemType {};
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}
        let saturation = 7;
        let item = Item::with_data(&item_type, SaturationData { saturation });
        assert_eq!(*item.item_type, item_type);
        assert_eq!(item.get_data::<SaturationData>().saturation, saturation);
    }

    #[test]
    fn create_item_with_multiple_data() {
        let item_type = ItemType {};
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
        assert_eq!(item.get_data::<SaturationData>().saturation, saturation);
        assert_eq!(item.get_data::<SpeedData>().speed, speed);
    }
}
