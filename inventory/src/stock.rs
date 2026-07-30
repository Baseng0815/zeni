//! On-hand stock, tracked in lots.
//!
//! A lot is a quantity of one [`crate::item`] at one [`crate::location`] with
//! its own acquisition date and expiry. Tracking lots rather than a single total
//! is what lets kane answer "what expires this week" and consume oldest-first.
//! The current level of an item is the sum of its open lots.
//!
//! Planned surface: a `Lot` (id, item, location, `Quantity`, acquired date,
//! optional expiry) and level queries that roll lots up per item or location.
