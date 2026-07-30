//! Expense tracking and the ledger.
//!
//! This crate owns the financial half of kane. It is deliberately free of any
//! transport or storage concerns: the types here describe *what* an expense or a
//! ledger entry is, and [`store`] describes the persistence contract that a
//! host (currently the `web` crate's server half) has to satisfy.
//!
//! Layering, bottom-up:
//!
//! - [`money`] — the amount/currency primitive everything else is denominated in.
//! - [`account`] — the named buckets postings move value between.
//! - [`category`] — the user-facing classification of spending.
//! - [`expense`] — a single recorded outflow, the primary thing a user enters.
//! - [`ledger`] — the append-only double-entry record and its balance queries.
//! - [`report`] — aggregations derived from the ledger (per period, per category).
//! - [`store`] — persistence traits.
//! - [`error`] — the crate's error type.

pub mod account;
pub mod category;
pub mod error;
pub mod expense;
pub mod ledger;
pub mod money;
pub mod report;
pub mod store;
