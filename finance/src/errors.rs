use crate::account::AccountId;
use crate::ledger::store::LedgerStoreError;
use crate::money::Currency;

#[derive(Debug, thiserror::Error)]
pub enum FinanceError {
    #[error("transaction has no entries")]
    EmptyTransaction,
    #[error("transaction entries do not balance")]
    UnbalancedTransaction,
    #[error("currencies are different")]
    DifferentCurrencies,
    #[error("ledger store error: {0}")]
    LedgerStoreError(#[from] LedgerStoreError)
}

pub type FinanceResult<T = ()> = Result<T, FinanceError>;
