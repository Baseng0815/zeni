use kane_finance::account::Account;
use kane_finance::ledger::Ledger;
use kane_finance::money::{
    Currency,
    Money,
};
use kane_finance::transaction::TransactionEntry;

fn eur(cents: i128) -> Money {
    Money::new(cents, Currency::EUR)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ledger = Ledger::new();

    let checking = ledger.add_account(Account::new("Checking".to_owned(), eur(150_000)));
    let salary = ledger.add_account(Account::new("Salary".to_owned(), eur(0)));
    let rent = ledger.add_account(Account::new("Rent".to_owned(), eur(0)));
    let groceries = ledger.add_account(Account::new("Groceries".to_owned(), eur(0)));

    ledger.post(
        "July salary".to_owned(),
        vec![
            TransactionEntry::new(salary, eur(-320_000)),
            TransactionEntry::new(checking, eur(320_000)),
        ],
    )?;

    ledger.post(
        "July rent".to_owned(),
        vec![
            TransactionEntry::new(checking, eur(-95_000)),
            TransactionEntry::new(rent, eur(95_000)),
        ],
    )?;

    ledger.post(
        "Weekly groceries".to_owned(),
        vec![
            TransactionEntry::new(checking, eur(-6_387)),
            TransactionEntry::new(groceries, eur(6_387)),
        ],
    )?;

    println!("{ledger}");

    ledger.save_as_graph("ledger.dot")?;
    println!("\ngraph written to ledger.dot");

    Ok(())
}
