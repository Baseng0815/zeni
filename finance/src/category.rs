//! Spending categories.
//!
//! The classification a user actually thinks in ("groceries", "rent") as
//! opposed to the ledger accounts it maps onto. Kept separate from
//! [`crate::account`] so categories can be renamed or reorganised without
//! rewriting ledger history.
//!
//! Planned surface: a `Category` with an id, name, optional parent for
//! hierarchy, and the expense account it posts to.
