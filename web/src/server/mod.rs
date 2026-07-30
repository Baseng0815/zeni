//! Native-only server wiring.
//!
//! Compiled only under the `server` feature. This is the composition root: it
//! owns the concrete store implementations and hands them to the server
//! functions in [`crate::api`] via dioxus context, so the domain crates stay
//! ignorant of both HTTP and the database.
//!
//! Planned surface:
//!
//! - An `AppState` holding the `finance` and `inventory` stores plus config.
//! - Account handling — registration, login, sessions — and the per-request user
//!   identity that scopes every store query.
//! - Whatever `dioxus::launch` needs to attach the above to the axum router.
//!
//! No storage backend is chosen yet; the stores are the trait objects described
//! in `finance::store` and `inventory::store`.
