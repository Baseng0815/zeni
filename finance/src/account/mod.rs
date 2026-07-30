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

#[derive(Debug)]
pub struct Account {
    id: AccountId,
    description: String,
    created_at: Timestamp,
    balance: Money,
}

impl Account {
    pub fn new(
        description: String,
        balance: Money,
    ) -> Self {
        let created_at = Timestamp::now();
        let id = AccountId::from(Uuid::new_v7(uuid_timestamp(created_at)));

        Self {
            id,
            description,
            created_at,
            balance,
        }
    }

    pub fn id(&self) -> AccountId {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> Timestamp {
        self.created_at
    }

    pub fn balance(&self) -> Money {
        self.balance
    }

    pub(crate) fn set_balance(
        &mut self,
        balance: Money,
    ) {
        self.balance = balance;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, From, Into)]
pub struct AccountId(Uuid);
