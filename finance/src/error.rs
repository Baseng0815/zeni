//! The crate's error type.
//!
//! One `thiserror` enum for the whole crate, covering the failures the domain
//! can actually produce: unbalanced transactions, currency mismatches, unknown
//! account or category ids, and store failures wrapped from the host.
