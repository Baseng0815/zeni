use crate::account::{
    Account,
    AccountId,
};
use crate::errors::{
    FinanceError,
    FinanceResult,
};
use crate::money::Money;
use crate::transaction::{
    Transaction,
    TransactionEntry,
    TransactionId,
};

mod display;
mod graph;

#[derive(Debug, Default)]
pub struct Ledger {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn account(
        &self,
        id: AccountId,
    ) -> Option<&Account> {
        self.accounts.iter().find(|account| account.id() == id)
    }

    pub fn add_account(
        &mut self,
        account: Account,
    ) -> AccountId {
        let id = account.id();
        self.accounts.push(account);
        id
    }

    pub fn post(
        &mut self,
        description: String,
        entries: Vec<TransactionEntry>,
    ) -> FinanceResult<TransactionId> {
        let transaction = Transaction::new(description, entries)?;
        let updates = self.updated_balances(&transaction)?;

        for (index, balance) in updates {
            if let Some(account) = self.accounts.get_mut(index) {
                account.set_balance(balance);
            }
        }

        let id = transaction.id();
        self.transactions.push(transaction);

        Ok(id)
    }

    fn updated_balances(
        &self,
        transaction: &Transaction,
    ) -> FinanceResult<Vec<(usize, Money)>> {
        let mut updates: Vec<(usize, Money)> = Vec::new();

        for entry in transaction.entries() {
            let (index, account) = self
                .accounts
                .iter()
                .enumerate()
                .find(|(_, account)| account.id() == entry.account())
                .ok_or(FinanceError::UnknownAccount(entry.account()))?;

            match updates.iter_mut().find(|(updated, _)| *updated == index) {
                Some((_, balance)) => *balance = balance.try_add(*entry.amount())?,
                None => updates.push((index, account.balance().try_add(*entry.amount())?)),
            }
        }

        Ok(updates)
    }
}
