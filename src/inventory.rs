use crate::Item;

pub trait Inventory<'a> {
    fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>>;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{Inventory, Item, ItemData, ItemType};

    struct SimpleInventory<'a> {
        pub items: Vec<Item<'a>>,
    }
    impl<'a> Inventory<'a> for SimpleInventory<'a> {
        fn add_item(&mut self, item: Item<'a>) -> Option<Item<'a>> {
            self.items.push(item);
            None
        }
    }

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
    }
}
