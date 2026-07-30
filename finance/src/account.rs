//! Accounts: the buckets that ledger postings move value between.
//!
//! Covers both the user's real accounts (a bank account, a wallet, a credit
//! card) and the notional ones double-entry needs so every transaction
//! balances (an expense account per category, income, opening balances).
//!
//! Planned surface: an `Account` with an id, display name, and `AccountKind`
//! (asset / liability / income / expense / equity) determining its normal
//! balance sign.
