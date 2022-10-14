// this is a edited version of https://github.com/leudz/shipyard/blob/master/src/add_component.rs

use std::any::TypeId;

use crate::{item::ItemDataMap, ItemData};

pub trait ItemDataHack {
    const CAPACITY: usize;

    fn add_data(self, map: &mut ItemDataMap);
}

impl ItemDataHack for () {
    const CAPACITY: usize = 0;

    #[inline]
    fn add_data(self, _map: &mut ItemDataMap) {}
}

impl<D: ItemData> ItemDataHack for D {
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

macro_rules! impl_add_itemdata {
    ( $(($type: ident, $index: tt))+ ) => {
        impl<$($type: ItemData,)+> ItemDataHack for ($($type,)+)
        {
            const CAPACITY: usize = <[()]>::len(&[$(replace_expr!(($index) ())),*]);
            fn add_data(self, map: &mut ItemDataMap) {
                $(
                    map.insert(TypeId::of::<$type>(), Box::new(self.$index));
                )+
            }
        }
    };
}

macro_rules! add_itemdata {
    ($(($type: ident, $index: tt))*;($type1: ident, $index1: tt) $(($queue_type: ident, $queue_index: tt))*) => {
        impl_add_itemdata![$(($type, $index))*];
        add_itemdata![$(($type, $index))* ($type1, $index1); $(($queue_type, $queue_index))*];
    };
    ($(($type: ident, $index: tt))*;) => {
        impl_add_itemdata![$(($type, $index))*];
    }
}

add_itemdata![(A, 0); (B, 1) (C, 2) (D, 3) (E, 4) (F, 5) (G, 6) (H, 7) (I, 8) (J, 9)];
