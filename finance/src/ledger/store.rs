use thiserror::Error;

use crate::account::{
    Account,
    AccountId,
};
use crate::transaction::{
    Transaction,
    TransactionId,
};

#[derive(Debug, thiserror::Error)]
pub enum LedgerStoreError {
    #[error("duplicate account id: {0}")]
    DuplicateAccountId(AccountId),
    #[error("duplicate transaction id: {0}")]
    DuplicateTransactionId(TransactionId),
    #[error("no such account: {0}")]
    NoSuchAccount(AccountId),
    #[error("no such transaction: {0}")]
    NoSuchTransaction(TransactionId),
    #[error("domain error: {0}")]
    Domain(String),
}

pub type LedgerStoreResult<T = ()> = Result<T, LedgerStoreError>;

pub trait LedgerStore {
    async fn insert_account(
        &mut self,
        account: Account,
    ) -> LedgerStoreResult;

    async fn get_account(
        &self,
        id: AccountId,
    ) -> LedgerStoreResult<Account>;

    async fn insert_transaction(
        &mut self,
        transaction: Transaction,
    ) -> LedgerStoreResult;

    async fn get_transaction(
        &self,
        id: TransactionId,
    ) -> LedgerStoreResult<Transaction>;
}

pub struct InMemoryLedgerStore {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl LedgerStore for InMemoryLedgerStore {
    async fn insert_account(
        &mut self,
        account: Account,
    ) -> LedgerStoreResult {
        if self.get_account(account.id()).await.is_ok() {
            Err(LedgerStoreError::DuplicateAccountId(account.id()))?;
        }

        self.accounts.push(account);
        Ok(())
    }

    async fn get_account(
        &self,
        id: AccountId,
    ) -> LedgerStoreResult<Account> {
        self.accounts
            .iter()
            .find(|account| account.id() == id)
            .cloned()
            .ok_or_else(|| LedgerStoreError::NoSuchAccount(id))
    }

    async fn insert_transaction(
        &mut self,
        transaction: Transaction,
    ) -> LedgerStoreResult {
        if self.get_transaction(transaction.id()).await.is_ok() {
            Err(LedgerStoreError::DuplicateTransactionId(transaction.id()))?;
        }

        self.transactions.push(transaction);
        Ok(())
    }

    async fn get_transaction(
        &self,
        id: TransactionId,
    ) -> LedgerStoreResult<Transaction> {
        self.transactions
            .iter()
            .find(|transaction| transaction.id() == id)
            .cloned()
            .ok_or_else(|| LedgerStoreError::NoSuchTransaction(id))
    }
}
