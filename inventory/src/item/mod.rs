use derive_more::{
    Display,
    From,
    Into,
};
use jiff::Timestamp;
use zeni_finance::uuid_timestamp;
use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub id: ItemId,
    pub description: String,
    pub quantity: Quantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Count(u64),
    Weight(u64),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct ItemId(Uuid);

impl Item {
    pub fn new(
        description: String,
        quantity: Quantity,
    ) -> Self {
        Self {
            id: Uuid::new_v7(uuid_timestamp(Timestamp::now())).into(),
            description,
            quantity,
        }
    }
}
