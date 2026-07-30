//! Units of measure and the quantity primitive.
//!
//! Groceries mix dimensions freely — 2 pieces of bread, 1.5 kg of potatoes,
//! 750 ml of milk — so a bare number is never enough. Quantities carry their
//! unit and arithmetic is unit-checked: adding mass to volume is an error, not
//! a silent nonsense result. As with money in the `finance` crate, fractional
//! amounts are stored as scaled integers rather than floats.
//!
//! Planned surface: a `Unit` (mass / volume / count, with a base unit per
//! dimension), a `Quantity` pairing a scaled integer with a `Unit`, and
//! conversion within a dimension.
