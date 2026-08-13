use derive_more::{
    Display,
    From,
    Into,
};
use jiff::Timestamp;
use uuid::Uuid;

use crate::account::AccountId;
use crate::errors::{
    FinanceError,
    FinanceResult,
};
use crate::money::Money;
use crate::uuid_timestamp;

mod display;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct TransactionEntry {
    pub account: AccountId,
    pub amount: Money,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct TransactionId(Uuid);
