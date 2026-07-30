//! The double-entry ledger.
//!
//! The append-only record every balance and report is derived from. A
//! transaction holds two or more postings whose signed amounts sum to zero;
//! nothing mutates or deletes a posted transaction, corrections are posted as
//! reversing entries so history stays auditable.
//!
//! Planned surface: a `Posting` (account + signed `Amount`), a `Transaction`
//! (id, date, description, postings) that validates it balances on
//! construction, and a `Ledger` exposing balance-at-date and account-history
//! queries.
