//! Storage locations.
//!
//! Where stock physically sits. Locations matter beyond bookkeeping: they imply
//! storage conditions, which is what makes expiry estimates and "check the
//! freezer first" behaviour possible.
//!
//! Planned surface: a `Location` with an id, name, optional parent for nesting
//! (kitchen → pantry → top shelf), and a storage condition (ambient / chilled /
//! frozen).
