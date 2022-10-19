use crate::Item;

pub trait Inventory<'a> {
    fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>>;
}

pub trait InventoryGetItem<'a, T> {
    fn get_item(&self, t: T) -> Option<&'a Item>;
    fn get_item_mut(&mut self, t: T) -> Option<&'a mut Item>;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{Inventory, Item, ItemData, ItemType};

    use super::InventoryGetItem;

    struct SimpleInventory<'a> {
        pub items: Vec<Item<'a>>,
    }
    impl<'a> Inventory<'a> for SimpleInventory<'a> {
        fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
            self.items.push(item);
            None
        }
    }
    impl<'a> InventoryGetItem<'a, usize> for SimpleInventory<'a> {
        fn get_item(&self, t: usize) -> Option<&'a Item> {
            self.items.get(t)
        }

        fn get_item_mut(&mut self, t: usize) -> Option<&'a mut Item> {
            self.items.get_mut(t)
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
}
