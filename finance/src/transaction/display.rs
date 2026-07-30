use std::fmt::{
    Display,
    Formatter,
};

use crate::transaction::{
    Transaction,
    TransactionEntry,
};

impl Display for Transaction {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} {}", self.created_at, self.description)?;

        for entry in &self.entries {
            write!(f, "\n  {entry}")?;
        }

        Ok(())
    }
}

impl Display for TransactionEntry {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.account, self.amount)
    }
}
