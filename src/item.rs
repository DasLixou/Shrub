use std::{any::TypeId, collections::HashMap};

use crate::{itemdata_reflection::ItemDataReflection, ItemData, ItemType};

pub(crate) type ItemDataMap = HashMap<TypeId, Box<dyn ItemData>>;

pub struct Item<'t> {
    pub item_type: &'t ItemType,
    data: ItemDataMap,
}

impl<'t> Item<'t> {
    /// Creates a new Item, the HashMap for the data will not allocate until it is first inserted into.
    pub(crate) fn new(item_type: &'t ItemType) -> Self {
        let data = HashMap::new();
        Item { item_type, data }
    }

    /// Creates a new Item with the given data for the item.
    pub(crate) fn with_data<D: ItemDataReflection>(item_type: &'t ItemType, item_data: D) -> Self {
        let mut data = HashMap::with_capacity(D::CAPACITY);
        item_data.add_data(&mut data);
        Item { item_type, data }
    }

    /// Creates a new Item and reserves the given amount of capacity for the data map.
    pub(crate) fn with_capacity(item_type: &'t ItemType, data_capacity: usize) -> Self {
        let data = HashMap::with_capacity(data_capacity);
        Item { item_type, data }
    }

    /// Adds the given data to the item. When data with the given datatype already exists, it will be completly overriden.
    ///
    /// # Examples
    /// ```
    /// use shrub::{ItemData, ItemType};
    ///
    /// struct CoolData {
    ///     is_cool: bool,
    /// }
    /// impl ItemData for CoolData {}
    ///
    /// let item_type = ItemType::new();
    /// let mut item = item_type.item_with_capacity(1);
    /// item.add_data(CoolData { is_cool: true });
    /// assert_eq!(item.get_data::<CoolData>().unwrap().is_cool, true);
    /// ```
    #[inline]
    pub fn add_data<D: ItemDataReflection>(&mut self, item_data: D) {
        self.data.reserve(D::CAPACITY);
        item_data.add_data(&mut self.data);
    }

    /// Borrows data of the given datatype from the item. When the item doesn't have data from this datatype, it searches in the `ItemType`.
    ///
    /// # Examples
    /// ```
    /// use shrub::{ItemData, ItemType};
    ///
    /// struct CoolData {
    ///     is_cool: bool,
    /// }
    /// impl ItemData for CoolData {}
    ///
    /// let item_type = ItemType::with_data(CoolData { is_cool: true });
    /// let item = item_type.item_new();
    /// assert_eq!(item.get_data::<CoolData>().unwrap().is_cool, true); // this first searches in the item, then in the itemtype
    /// ```
    #[inline]
    pub fn get_data<D: ItemData>(&self) -> Option<&D> {
        match self.data.get(&TypeId::of::<D>()) {
            Some(d) => Some(d.as_any().downcast_ref::<D>().unwrap()),
            None => self.item_type.get_data::<D>(),
        }
    }

    /// Borrows data fo the given datatype from the item as mutable.
    ///
    /// # Attention
    /// Unlike `get_data`, `get_data_mut` can only borrow data from this item and will return `None` when the item doesnt include this Data and WON'T SEARCH IN THE ITEMTYPE for it.
    ///
    /// # Examples
    /// ```
    /// use shrub::{ItemData, ItemType};
    ///
    /// struct CoolData {
    ///     is_cool: bool,
    /// }
    /// impl ItemData for CoolData {}
    ///
    /// let item_type = ItemType::new();
    /// let mut item = item_type.item_with_data(CoolData { is_cool: true });
    /// assert_eq!(item.get_data::<CoolData>().unwrap().is_cool, true);
    /// item.get_data_mut::<CoolData>().unwrap().is_cool = false;
    /// assert_eq!(item.get_data::<CoolData>().unwrap().is_cool, false);
    /// ```
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
        let item_type = ItemType::new();
        let item = Item::with_capacity(&item_type, 0);
        assert_eq!(*item.item_type, item_type);
    }

    #[test]
    fn create_item_with_single_data() {
        let item_type = ItemType::new();
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
        let item_type = ItemType::new();
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
        let item_type = ItemType::new();
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
