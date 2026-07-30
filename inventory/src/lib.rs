//! Inventory and stock management.
//!
//! This crate owns the physical half of kane: what the user has, how much of
//! it, where it is, and when it expires. Groceries are the driving use case, so
//! quantities are unit-aware and stock is tracked in lots rather than as a
//! single running total — 500 g of flour bought today is not interchangeable
//! with 500 g that expires next week.
//!
//! Like `finance`, it is free of transport and storage concerns.
//!
//! Layering, bottom-up:
//!
//! - [`unit`] — units of measure and the unit-checked quantity primitive.
//! - [`item`] — the definition of a thing that can be stocked.
//! - [`location`] — where stock physically sits (pantry, fridge, freezer).
//! - [`stock`] — on-hand lots, their quantities and expiry.
//! - [`movement`] — the append-only record of stock entering and leaving.
//! - [`shopping`] — restock suggestions derived from stock levels.
//! - [`store`] — persistence traits.
//! - [`error`] — the crate's error type.
//!
//! Note on naming: the package is `kane-inventory` so it cannot be confused
//! with the registry crate `inventory`; dependents alias it back to `inventory`.

pub mod error;
pub mod item;
pub mod location;
pub mod movement;
pub mod shopping;
pub mod stock;
pub mod store;
pub mod unit;
