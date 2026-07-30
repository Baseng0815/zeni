use crate::errors::{
    FinanceError,
    FinanceResult,
};

mod display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    amount: i128,
    currency: Currency,
}

impl Money {
    pub fn new(
        amount: i128,
        currency: Currency,
    ) -> Self {
        Self { amount, currency }
    }

    pub fn amount(&self) -> i128 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    pub fn try_add(
        self,
        other: Self,
    ) -> FinanceResult<Self> {
        if self.currency != other.currency {
            return Err(FinanceError::CurrencyMismatch {
                expected: self.currency,
                actual: other.currency,
            });
        }

        let amount = self
            .amount
            .checked_add(other.amount)
            .ok_or(FinanceError::AmountOverflow)?;

        Ok(Self {
            amount,
            currency: self.currency,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    EUR,
}
