//! One module per route in [`crate::Route`].
//!
//! Views are placeholders for now — they establish the routing structure and
//! nothing more. Each will grow to fetch its data through [`crate::api`], using
//! `use_server_future` rather than `use_resource` so the server-rendered and
//! hydrated markup agree.

mod account;
mod expenses;
mod home;
mod stock;

pub use account::Account;
pub use expenses::Expenses;
pub use home::Home;
pub use stock::Stock;
