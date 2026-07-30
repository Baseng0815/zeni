//! The client/server boundary.
//!
//! Everything the browser can ask the server to do goes through a server
//! function here. In dioxus 0.7 those are declared with the `#[get]` / `#[post]`
//! attribute macros, which generate a real HTTP endpoint on the server and a
//! matching request stub on the client:
//!
//! ```ignore
//! #[get("/api/expenses")]
//! async fn list_expenses() -> Result<Vec<ExpenseDto>> { /* server-only body */ }
//! ```
//!
//! Two rules keep this boundary honest:
//!
//! - Arguments and return types are DTOs defined in this module, not domain
//!   types from `finance` / `inventory`. The domain crates are server-only, so a
//!   domain type in a signature would drag them into the wasm build.
//! - Function bodies are the only place server-only code belongs. Imports they
//!   need go inside the function or behind `#[cfg(feature = "server")]`.
//!
//! Planned submodules: `expenses`, `stock`, and `auth`, each pairing its DTOs
//! with the server functions that produce them.
