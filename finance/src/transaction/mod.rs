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

use crate::account::AccountId;
use crate::errors::{
    FinanceError,
    FinanceResult,
};
use crate::money::Money;

mod display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub(crate) id: TransactionId,
    pub(crate) created_at: Timestamp,
    pub(crate) description: String,
    pub(crate) entries: Vec<TransactionEntry>,
}

impl Transaction {
    pub fn id(&self) -> TransactionId {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn entries(&self) -> &[TransactionEntry] {
        &self.entries
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TransactionEntry {
    pub account: AccountId,
    pub amount: Money,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into, Serialize, Deserialize)]
pub struct TransactionId(Uuid);
