use derive_more::{
    Display,
    From,
    Into,
};
use jiff::Timestamp;
use uuid::Uuid;

pub mod display;

#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) id: ItemId,
    pub(crate) created_at: Timestamp,
    pub(crate) description: String,
    pub(crate) r#type: ItemType,
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ItemType {
    Grocery,
    Drink,
    Household,
    Unknown,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct ItemId(Uuid);

impl Item {
    pub fn id(&self) -> ItemId {
        self.id
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn r#type(&self) -> ItemType {
        self.r#type
    }
}
