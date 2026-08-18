use crate::money::Money;
use derive_more::{
    Display,
    From,
    Into,
};
use jiff::Timestamp;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

pub mod display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub(crate) id: AccountId,
    pub(crate) created_at: Timestamp,
    pub(crate) description: String,
    pub(crate) r#type: AccountType,
}

#[derive(Debug, Clone, Copy, Display, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Income,
    Expense,
    Equity,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into, Serialize, Deserialize)]
pub struct AccountId(Uuid);

impl Account {
    pub fn id(&self) -> AccountId {
        self.id
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn r#type(&self) -> &AccountType {
        &self.r#type
    }
}
