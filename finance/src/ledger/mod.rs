use jiff::Timestamp;
use uuid::Uuid;

use crate::account::{
    Account,
    AccountId,
    AccountType,
};
use crate::errors::{
    FinanceError,
    FinanceResult,
};
use crate::ledger::store::LedgerStore;
use crate::transaction::{
    Transaction,
    TransactionEntry,
    TransactionId,
};

pub mod store;

pub struct Ledger<S> {
    store: S,
}

impl<S: LedgerStore> Ledger<S> {
    pub async fn create_account(
        &mut self,
        description: String,
        r#type: AccountType,
    ) -> FinanceResult<Account> {
        let created_at = Timestamp::now();
        let id = AccountId::from(Uuid::new_v7(uuid_timestamp(created_at)));

        let account = Account {
            id,
            created_at,
            description,
            r#type,
        };

        self.store.insert_account(account.clone()).await?;
        Ok(account)
    }

    pub async fn create_transaction(
        &mut self,
        description: String,
        entries: Vec<TransactionEntry>,
    ) -> FinanceResult<Transaction> {
        self.error_if_unbalanced(&entries)?;

        let created_at = Timestamp::now();
        let id = TransactionId::from(Uuid::new_v7(uuid_timestamp(created_at)));

        let transaction = Transaction {
            id,
            created_at,
            description,
            entries,
        };

        self.store.insert_transaction(transaction.clone()).await?;
        Ok(transaction)
    }

    fn error_if_unbalanced(
        &mut self,
        entries: &[TransactionEntry],
    ) -> FinanceResult {
        let amount_sum = entries
            .iter()
            .fold(0, |acc, entry| acc + entry.amount.amount);

        if amount_sum != 0 {
            Err(FinanceError::UnbalancedTransaction)?;
        }

        Ok(())
    }
}

fn uuid_timestamp(timestamp: Timestamp) -> uuid::Timestamp {
    let seconds = u64::try_from(timestamp.as_second()).unwrap_or_default();
    let nanos = u32::try_from(timestamp.subsec_nanosecond()).unwrap_or_default();

    uuid::Timestamp::from_unix(uuid::NoContext, seconds, nanos)
}
