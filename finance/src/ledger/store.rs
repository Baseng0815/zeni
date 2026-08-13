use thiserror::Error;

use crate::account::{
    Account,
    AccountId,
};
use crate::transaction::{
    Transaction,
    TransactionId,
};

pub enum LedgerStoreError<E = ()> {
    DuplicateAccountId(AccountId),
    DuplicateTransactionId(TransactionId),
    NoSuchAccount(AccountId),
    NoSuchTransaction(TransactionId),
    Domain(#[from] E),
}

pub type LedgerStoreResult<T = (), E = ()> = Result<T, LedgerStoreError<E>>;

pub trait LedgerStore {
    type DomainError;

    async fn insert_account(
        &mut self,
        account: Account,
    ) -> LedgerStoreResult<(), Self::DomainError>;

    async fn get_account(
        &mut self,
        id: AccountId,
    ) -> LedgerStoreResult<Account, Self::DomainError>;

    async fn insert_transaction(
        &mut self,
        transaction: Transaction,
    ) -> LedgerStoreResult<(), Self::DomainError>;

    async fn get_transaction(
        &mut self,
        id: TransactionId,
    ) -> LedgerStoreResult<Transaction, Self::DomainError>;
}

pub struct InMemoryLedgerStore {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl LedgerStore for InMemoryLedgerStore {
    type DomainError = ();

    async fn insert_account(
        &mut self,
        account: Account,
    ) -> LedgerStoreResult<(), Self::DomainError> {
        if self.get_account(account.id()).await.is_ok() {
            Err(LedgerStoreError::DuplicateAccountId(account.id()))?;
        }

        self.accounts.push(account);
        Ok(())
    }

    async fn get_account(
        &mut self,
        id: AccountId,
    ) -> LedgerStoreResult<Account, Self::DomainError> {
        self.accounts
            .iter()
            .find(|account| account.id() == id)
            .cloned()
            .ok_or_else(|| LedgerStoreError::NoSuchAccount(id))
    }

    async fn insert_transaction(
        &mut self,
        transaction: Transaction,
    ) -> LedgerStoreResult<(), Self::DomainError> {
        if self.get_transaction(transaction.id()).await.is_ok() {
            Err(LedgerStoreError::DuplicateTransactionId(transaction.id()))?;
        }

        self.transactions.push(transaction);
        Ok(())
    }

    async fn get_transaction(
        &mut self,
        id: TransactionId,
    ) -> LedgerStoreResult<Transaction, Self::DomainError> {
        self.transactions
            .iter()
            .find(|transaction| transaction.id() == id)
            .cloned()
            .ok_or_else(|| LedgerStoreError::NoSuchTransaction(id))
    }
}
