#![allow(unused_imports)]
use vstd::prelude::*;

use super::ItemTableStaticMetadata;

verus! {

#[verifier::ext_equal]
pub struct ItemTableSnapshot<I>
{
    pub m: IMap<u64, I>,
}

impl<I> ItemTableSnapshot<I>
{
    pub open spec fn init() -> Self
    {
        Self{ m: IMap::<u64, I>::empty() }
    }

    pub open spec fn create(&self, item_addr: u64, item: I) -> Self
    {
        Self{ m: self.m.insert(item_addr, item) }
    }

    pub open spec fn delete(&self, item_addr: u64) -> Self
    {
        Self{ m: self.m.remove(item_addr) }
    }
}

#[verifier::ext_equal]
pub struct ItemTableView<I>
{
    pub sm: ItemTableStaticMetadata,
    pub used_slots: int,
    pub durable: ItemTableSnapshot<I>,
    pub tentative: Option<ItemTableSnapshot<I>>,
}

}


