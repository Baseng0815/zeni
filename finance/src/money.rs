//! The monetary primitive.
//!
//! Every amount in kane is stored as an integer count of a currency's minor
//! unit (cents for EUR/USD) paired with its currency, never as a float — see
//! the `float_arithmetic` workspace lint. Rounding is therefore an explicit
//! operation rather than an accident of representation.
//!
//! Planned surface: an `Amount` (minor units + `Currency`), a `Currency` code,
//! and checked arithmetic that refuses to add across currencies.
