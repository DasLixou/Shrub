use crate::{itemdata::EmptyItemData, Item, ItemData};

// TODO: look whats actually required, these are just filled in
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ItemType {}

impl ItemType {
    pub fn create_empty(&self) -> Item<EmptyItemData> {
        Item::new_nodata(self)
    }

    pub fn create<D: ItemData>(&self, data: D) -> Item<D> {
        Item::new(self, data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ItemData, ItemType};

    #[test]
    fn create_item() {
        let item_type = ItemType {};
        let item = item_type.create_empty();
        assert_eq!(item.item_type, &item_type);
    }

    #[test]
    fn create_item_with_data() {
        struct SaturationData {
            saturation: i16,
        }
        impl ItemData for SaturationData {}

        let item_type = ItemType {};
        let saturation = 8;
        let item = item_type.create(SaturationData { saturation });

        assert_eq!(item.item_type, &item_type);
        assert_eq!(item.data.saturation, saturation);
    }
}
