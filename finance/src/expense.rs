//! Expenses: the primary record a user enters.
//!
//! An expense is the user-facing view of a purchase — when, how much, what
//! category, which account paid, and optionally which inventory items it
//! restocked. Recording one produces the balanced [`crate::ledger`] entry; the
//! expense is the input, the ledger entry is the derived truth.
//!
//! Planned surface: an `Expense` (id, date, payee, `Amount`, `Category`,
//! paying account, note) and its line items, so a single receipt can split
//! across categories and feed the `inventory` crate.
