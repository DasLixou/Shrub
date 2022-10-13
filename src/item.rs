use std::{any::TypeId, collections::HashMap};

use crate::{ItemData, ItemType};

pub struct Item<'t> {
    pub item_type: &'t ItemType,
    data: HashMap<TypeId, Box<dyn ItemData>>,
}

impl<'t> Item<'t> {
    pub(crate) fn new(item_type: &'t ItemType, data_capacity: usize) -> Self {
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
    use crate::{Item, ItemType};

    #[test]
    fn create_item() {
        let item_type = ItemType {};
        let item = Item::new(&item_type, 0);
        assert_eq!(*item.item_type, item_type);
    }
}
