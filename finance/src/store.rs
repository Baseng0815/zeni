//! Persistence contract.
//!
//! The crate stays storage-agnostic: it defines the traits a backing store must
//! implement and leaves the choice of database to the host. No implementation
//! is committed to yet — when one lands it goes behind these traits, so the
//! domain types above never learn about SQL.
//!
//! Planned surface: an async `LedgerStore` (append a transaction, read
//! transactions in a range, read balances) plus `AccountStore` / `CategoryStore`
//! for the reference data, and an in-memory implementation for tests.
