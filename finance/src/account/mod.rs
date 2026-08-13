use crate::money::Money;
use crate::uuid_timestamp;
use derive_more::{
    Display,
    From,
    Into,
};
use jiff::Timestamp;
use uuid::Uuid;

mod display;

#[derive(Debug, Clone)]
pub struct Account {
    pub(crate) id: AccountId,
    pub(crate) created_at: Timestamp,
    pub(crate) description: String,
    pub(crate) r#type: AccountType,
}

#[derive(Debug, Clone, Copy, Display)]
pub enum AccountType {
    Asset,
    Liability,
    Income,
    Expense,
    Equity,
}

impl Account {
    pub fn id(&self) -> AccountId {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn r#type(&self) -> &AccountType {
        &self.r#type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct AccountId(Uuid);
