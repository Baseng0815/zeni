//! Persistence contract.
//!
//! Mirrors `finance`'s approach: traits only, no database chosen yet, so the
//! domain types never learn about SQL.
//!
//! Planned surface: async `ItemStore`, `LocationStore`, and a `StockStore` that
//! appends movements and reads lots, plus in-memory implementations for tests.
