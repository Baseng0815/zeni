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

#[derive(Debug)]
pub struct Transaction {
    id: TransactionId,
    description: String,
    created_at: Timestamp,
    entries: Vec<TransactionEntry>,
}

impl Transaction {
    pub(crate) fn new(
        description: String,
        entries: Vec<TransactionEntry>,
    ) -> FinanceResult<Self> {
        let created_at = Timestamp::now();
        let id = TransactionId::from(Uuid::new_v7(uuid_timestamp(created_at)));

        ensure_balanced(&entries)?;

        Ok(Self {
            id,
            description,
            created_at,
            entries,
        })
    }

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

fn ensure_balanced(entries: &[TransactionEntry]) -> FinanceResult {
    if entries.is_empty() {
        return Err(FinanceError::EmptyTransaction);
    }

    let mut totals: Vec<Money> = Vec::new();

    for entry in entries {
        match totals
            .iter_mut()
            .find(|total| total.currency() == entry.amount.currency())
        {
            Some(total) => *total = total.try_add(entry.amount)?,
            None => totals.push(entry.amount),
        }
    }

    if totals.iter().any(|total| total.amount() != 0) {
        return Err(FinanceError::UnbalancedTransaction);
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct TransactionEntry {
    account: AccountId,
    amount: Money,
}

impl TransactionEntry {
    pub fn new(
        account: AccountId,
        amount: Money,
    ) -> Self {
        Self { account, amount }
    }

    pub fn account(&self) -> AccountId {
        self.account
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct TransactionId(Uuid);
