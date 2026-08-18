use serde::{
    Deserialize,
    Serialize,
};

use crate::errors::{
    FinanceError,
    FinanceResult,
};

mod display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: i128,
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    EUR,
}

impl Money {
    pub fn try_add(
        self,
        other: Money,
    ) -> FinanceResult<Money> {
        if self.currency != other.currency {
            Err(FinanceError::DifferentCurrencies)?;
        }

        Ok(Money {
            amount: self.amount + other.amount,
            ..self
        })
    }
}
