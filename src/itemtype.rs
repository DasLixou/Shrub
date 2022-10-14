use crate::{utils::add_data::ItemDataHack, Item};

// TODO: look whats actually required, these are just filled in
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ItemType {}

impl ItemType {
    pub fn item_with_data<D: ItemDataHack>(&self, item_data: D) -> Item {
        Item::with_data(self, item_data)
    }

    pub fn item_with_capacity(&self, data_capacity: usize) -> Item {
        Item::with_capacity(self, data_capacity)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ItemData, ItemType};

    #[test]
    fn create_item() {
        let item_type = ItemType {};
        let item = item_type.item_with_capacity(0);
        assert_eq!(item.item_type, &item_type);
    }

    #[test]
    fn create_item_with_capacity_data() {
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}

        let item_type = ItemType {};
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

        let item_type = ItemType {};
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
}
