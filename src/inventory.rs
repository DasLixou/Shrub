use crate::Item;

/// Trait to mark struct as inventory and provide basic function api
pub trait Inventory<'a> {
    /// Adds an item to the inventory.
    /// * when the inventory for some reason can't pickup the item, it will return the item in `Some(Item)`
    ///
    /// # Examples
    /// ```
    /// use std::vec;
    /// use shrub::{Inventory, InventorySelector, Item, ItemData, ItemType};
    ///
    /// struct SimpleInventory<'a> {
    ///     items: Vec<Item<'a>>,
    /// }
    /// impl<'a> Inventory<'a> for SimpleInventory<'a> {
    ///     fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
    ///         self.items.push(item);
    ///         None
    ///     }
    /// }
    ///
    /// let mut inventory = SimpleInventory { items: vec![] };
    /// let item_type = ItemType::new();
    /// let item = item_type.item_new();
    /// if let Some(i) = inventory.add_item(item) {
    ///     println!("Couldn't add item to inventory");
    /// }
    /// ```
    fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>>;
}

/// Trait to implement selecting an item from an inventory
pub trait InventorySelector<'a, S> {
    /// Borrows an item with the specified selector
    ///
    /// # Examples
    /// ```
    /// # use std::vec;
    /// use shrub::{Inventory, InventorySelector, Item, ItemData, ItemType};
    ///
    /// struct SimpleInventory<'a> {
    ///     items: Vec<Item<'a>>,
    /// }
    /// # impl<'a> Inventory<'a> for SimpleInventory<'a> {
    /// #     fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
    /// #         self.items.push(item);
    /// #         None
    /// #     }
    /// # }
    /// impl<'a> InventorySelector<'a, usize> for SimpleInventory<'a> {
    ///     fn get_item(&self, selector: usize) -> Option<&'a Item> {
    ///         self.items.get(selector)
    ///     }
    ///     
    ///     // ...
    ///     # fn get_item_mut(&mut self, selector: usize) -> Option<&'a mut Item> {
    ///     #     self.items.get_mut(selector)
    ///     # }
    ///     #
    ///     # fn remove_item(&mut self, selector: usize) -> Option<Item<'a>> {
    ///     #     if selector < self.items.len() {
    ///     #         Some(self.items.remove(selector))
    ///     #     } else {
    ///     #         None
    ///     #     }
    ///     # }
    /// }
    ///
    /// let mut inventory = SimpleInventory { items: vec![] };
    /// let item_type = ItemType::new();
    /// let item = item_type.item_new();
    /// inventory.add_item(item);
    ///
    /// let item = inventory.get_item(0);
    /// ```
    fn get_item(&self, selector: S) -> Option<&'a Item>;

    /// Borrows an item as mutable with the specified selector
    ///
    /// # Examples
    /// ```
    /// # use std::vec;
    /// use shrub::{Inventory, InventorySelector, Item, ItemData, ItemType};
    ///
    /// struct SimpleInventory<'a> {
    ///     items: Vec<Item<'a>>,
    /// }
    /// # impl<'a> Inventory<'a> for SimpleInventory<'a> {
    /// #     fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
    /// #         self.items.push(item);
    /// #         None
    /// #     }
    /// # }
    /// impl<'a> InventorySelector<'a, usize> for SimpleInventory<'a> {
    ///     fn get_item_mut(&mut self, selector: usize) -> Option<&'a mut Item> {
    ///         self.items.get_mut(selector)
    ///     }
    ///     
    ///     // ...
    ///     # fn get_item(&self, selector: usize) -> Option<&'a Item> {
    ///     #     self.items.get(selector)
    ///     # }
    ///     #
    ///     # fn remove_item(&mut self, selector: usize) -> Option<Item<'a>> {
    ///     #     if selector < self.items.len() {
    ///     #         Some(self.items.remove(selector))
    ///     #     } else {
    ///     #         None
    ///     #     }
    ///     # }
    /// }
    ///
    /// let mut inventory = SimpleInventory { items: vec![] };
    /// let item_type = ItemType::new();
    /// let item = item_type.item_new();
    /// inventory.add_item(item);
    ///
    /// let mut item = inventory.remove_item(0);
    /// ```
    fn get_item_mut(&mut self, selector: S) -> Option<&'a mut Item>;

    /// Removes an item with the specified selector
    ///
    /// # Examples
    /// ```
    /// # use std::vec;
    /// use shrub::{Inventory, InventorySelector, Item, ItemData, ItemType};
    ///
    /// struct SimpleInventory<'a> {
    ///     items: Vec<Item<'a>>,
    /// }
    /// # impl<'a> Inventory<'a> for SimpleInventory<'a> {
    /// #     fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
    /// #         self.items.push(item);
    /// #         None
    /// #     }
    /// # }
    /// impl<'a> InventorySelector<'a, usize> for SimpleInventory<'a> {
    ///     fn remove_item(&mut self, selector: usize) -> Option<Item<'a>> {
    ///         if selector < self.items.len() {
    ///             Some(self.items.remove(selector))
    ///         } else {
    ///             None
    ///         }
    ///     }
    ///
    ///     // ...
    ///     # fn get_item_mut(&mut self, selector: usize) -> Option<&'a mut Item> {
    ///     #     self.items.get_mut(selector)
    ///     # }
    ///     # fn get_item(&self, selector: usize) -> Option<&'a Item> {
    ///     #     self.items.get(selector)
    ///     # }
    /// }
    ///
    /// let mut inventory = SimpleInventory { items: vec![] };
    /// let item_type = ItemType::new();
    /// let item = item_type.item_new();
    /// inventory.add_item(item);
    ///
    /// let mut item = inventory.get_item_mut(0);
    /// ```
    fn remove_item(&mut self, selector: S) -> Option<Item<'a>>;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{Inventory, InventorySelector, Item, ItemData, ItemType};

    struct SimpleInventory<'a> {
        pub items: Vec<Item<'a>>,
    }
    impl<'a> Inventory<'a> for SimpleInventory<'a> {
        fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
            self.items.push(item);
            None
        }
    }
    impl<'a> InventorySelector<'a, usize> for SimpleInventory<'a> {
        fn get_item(&self, selector: usize) -> Option<&'a Item> {
            self.items.get(selector)
        }

        fn get_item_mut(&mut self, selector: usize) -> Option<&'a mut Item> {
            self.items.get_mut(selector)
        }

        fn remove_item(&mut self, selector: usize) -> Option<Item<'a>> {
            if selector < self.items.len() {
                Some(self.items.remove(selector))
            } else {
                None
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    enum Category {
        Weapon,
        Food,
        Block,
    }
    struct CategoryData(Category);
    impl ItemData for CategoryData {}

    #[test]
    fn create_basic_inventory() {
        let mut inventory = SimpleInventory { items: vec![] };
        let item_type = ItemType::new();
        let item = item_type.item_new();

        assert_eq!(inventory.items.len(), 0);
        assert!(inventory.add_item(item).is_none());
        assert_eq!(inventory.items.len(), 1);
    }

    #[test]
    fn create_nested_inventory() {
        struct NestedInventory<'a> {
            pub weapons: SimpleInventory<'a>,
            pub food: SimpleInventory<'a>,
            pub blocks: SimpleInventory<'a>,
        }
        impl<'a> Inventory<'a> for NestedInventory<'a> {
            fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
                match item.get_data::<CategoryData>()?.0 {
                    Category::Weapon => self.weapons.add_item(item),
                    Category::Food => self.food.add_item(item),
                    Category::Block => self.blocks.add_item(item),
                }
            }
        }

        let mut nested_inventory = NestedInventory {
            weapons: SimpleInventory { items: vec![] },
            food: SimpleInventory { items: vec![] },
            blocks: SimpleInventory { items: vec![] },
        };
        assert!(nested_inventory.weapons.items.is_empty());
        assert!(nested_inventory.food.items.is_empty());
        assert!(nested_inventory.blocks.items.is_empty());

        let wooden_sword = ItemType::with_data(CategoryData(Category::Weapon));
        let item = wooden_sword.item_new();
        nested_inventory.add_item(item);

        assert!(nested_inventory.weapons.items.len() == 1);
        assert!(nested_inventory.food.items.is_empty());
        assert!(nested_inventory.blocks.items.is_empty());

        assert_eq!(
            nested_inventory
                .weapons
                .get_item(0)
                .unwrap()
                .get_data::<CategoryData>()
                .unwrap()
                .0,
            Category::Weapon
        );
    }

    #[test]
    fn remove_item() {
        let mut inventory = SimpleInventory { items: vec![] };
        let item_type = ItemType::new();
        let item = item_type.item_new();

        assert_eq!(inventory.items.len(), 0);
        assert!(inventory.add_item(item).is_none());
        assert_eq!(inventory.items.len(), 1);
        assert!(inventory.remove_item(0).is_some());
        assert_eq!(inventory.items.len(), 0);
    }
}
