use crate::account::AccountId;
use crate::money::Currency;

#[derive(Debug, thiserror::Error)]
pub enum FinanceError {
    #[error("transaction has no entries")]
    EmptyTransaction,
    #[error("transaction entries do not balance")]
    UnbalancedTransaction,
    #[error("unknown account {0}")]
    UnknownAccount(AccountId),
    #[error("currency mismatch: expected {expected}, got {actual}")]
    CurrencyMismatch { expected: Currency, actual: Currency },
    #[error("amount overflow")]
    AmountOverflow,
}

pub type FinanceResult<T = ()> = Result<T, FinanceError>;
