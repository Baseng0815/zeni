use std::fmt::{
    Display,
    Formatter,
};

use crate::ledger::Ledger;

impl Display for Ledger {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "Accounts:")?;

        for account in &self.accounts {
            write!(f, "\n  {account}")?;
        }

        write!(f, "\nTransactions:")?;

        for transaction in &self.transactions {
            write!(f, "\n  {} {}", transaction.created_at(), transaction.description())?;

            for entry in transaction.entries() {
                match self.account(entry.account()) {
                    Some(account) => write!(f, "\n    {}: {}", account.description(), entry.amount())?,
                    None => write!(f, "\n    {}: {}", entry.account(), entry.amount())?,
                }
            }
        }

        Ok(())
    }
}
