use crate::Item;

// TODO: look whats actually required, these are just filled in
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ItemType {}

impl ItemType {
    pub fn create(&self, data_capacity: usize) -> Item {
        Item::new(self, data_capacity)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ItemData, ItemType};

    #[test]
    fn create_item() {
        let item_type = ItemType {};
        let item = item_type.create(0);
        assert_eq!(item.item_type, &item_type);
    }

    #[test]
    fn create_item_with_data() {
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        let item_type = ItemType {};
        let saturation = 8;
        let mut item = item_type.create(1);
        item.add_data(SaturationData { saturation });

        assert_eq!(item.item_type, &item_type);
        assert_eq!(item.get_data::<SaturationData>().saturation, saturation);
    }
}
