use std::any::TypeId;

use crate::{item::ItemDataMap, ItemData};

pub trait ItemDataReflection {
    const CAPACITY: usize;

    fn add_data(self, map: &mut ItemDataMap);
}

impl ItemDataReflection for () {
    const CAPACITY: usize = 0;

    #[inline]
    fn add_data(self, _map: &mut ItemDataMap) {
        println!("{}", (12, 13).1);
    }
}

impl<D: ItemData> ItemDataReflection for D {
    const CAPACITY: usize = 1;

    #[inline]
    fn add_data(self, map: &mut ItemDataMap) {
        map.insert(TypeId::of::<D>(), Box::new(self));
    }
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! impl_itemdata_reflection {
    ( $(($generic:ident, $index:tt))+ ) => {
        impl<$($generic: ItemData),+> ItemDataReflection for ($($generic,)+) {
            const CAPACITY: usize = <[()]>::len(&[$(replace_expr!(($generic) ())),*]);

            #[inline]
            fn add_data(self, map: &mut ItemDataMap) {
                $(
                    map.insert(TypeId::of::<$generic>(), Box::new(self.$index));
                )+
            }
        }
    };
}

impl_itemdata_reflection!((A, 0));
impl_itemdata_reflection!((A, 0)(B, 1));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8));
impl_itemdata_reflection!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)(J, 9));
