//! Stock movements: the append-only record of change.
//!
//! Every change to a lot is a movement — purchased, consumed, discarded,
//! relocated, corrected — so stock levels are a fold over movements rather than
//! a mutable number. Same discipline as the finance ledger, and for the same
//! reason: the history has to survive corrections.
//!
//! Purchases are where the two halves of kane meet: a movement can reference
//! the `finance` expense that paid for it, without this crate depending on
//! `finance` — the id is opaque here and resolved by the `web` crate.
//!
//! Planned surface: a `Movement` (id, lot, `MovementKind`, `Quantity`, when,
//! optional external expense reference) and the fold that produces current
//! levels.
