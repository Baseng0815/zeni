//! Restock suggestions.
//!
//! Derives what needs buying by comparing current levels from
//! [`crate::stock`] against the minimum levels on [`crate::item`], and by
//! surfacing lots that are about to expire. Pure and read-only — building a list
//! changes no stock; only the resulting purchase does, via
//! [`crate::movement`].
//!
//! Planned surface: a `ShoppingList` of suggested items with the shortfall
//! quantity and why it was suggested (below minimum / expiring / depleted).
