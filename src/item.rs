use crate::{itemdata::EmptyItemData, ItemData, ItemType};

pub struct Item<'t, D: ItemData> {
    pub item_type: &'t ItemType,
    pub data: D,
}

// TODO: maybe remove and make it a feature
impl<'t> Item<'t, EmptyItemData> {
    pub(crate) fn new_nodata(item_type: &'t ItemType) -> Item<EmptyItemData> {
        Item {
            item_type,
            data: EmptyItemData {},
        }
    }
}

impl<'t, D: ItemData> Item<'t, D> {
    pub(crate) fn new(item_type: &'t ItemType, data: D) -> Self {
        Item { item_type, data }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Item, ItemType};

    #[test]
    fn create_item() {
        let item_type = ItemType {};
        let item = Item::new_nodata(&item_type);
        assert_eq!(*item.item_type, item_type);
    }
}
