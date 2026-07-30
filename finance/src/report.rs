//! Aggregations over the ledger.
//!
//! Pure functions from ledger data to the summaries the UI shows: spend per
//! category over a period, month-over-month trends, account balances. Read-only
//! by construction — reports never post.
//!
//! Planned surface: a `Period` selector and per-category / per-account
//! breakdown types the `web` crate can hand straight to a view.
