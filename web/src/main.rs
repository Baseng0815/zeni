//! zeni's web application — both halves of it.
//!
//! This is a single dioxus-fullstack crate compiled twice: once to
//! wasm32-unknown-unknown for the browser (the `web` feature) and once natively
//! for the server (the `server` feature). `dx serve` drives both.
//!
//! The split that matters:
//!
//! - [`views`] and [`components`] are shared UI, compiled into both halves.
//! - [`api`] is the wire boundary. Its server functions have client stubs that
//!   issue HTTP requests and server bodies that do the real work, so it is the
//!   only place the two halves meet.
//! - `server` is native-only and holds the process wiring — app state, stores,
//!   sessions. It is where the `finance` and `inventory` crates are used; they
//!   are optional dependencies enabled by the `server` feature and never reach
//!   the wasm bundle.

use dioxus::prelude::*;

mod api;
mod components;
mod views;

#[cfg(feature = "server")]
mod server;

use components::Navbar;
use views::{
    Account,
    Expenses,
    Home,
    Stock,
};

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// The app's URL structure.
///
/// Every route sits under the [`Navbar`] layout, which renders the shared
/// chrome and an `Outlet` for the active route.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/expenses")]
        Expenses {},
        #[route("/stock")]
        Stock {},
        #[route("/account")]
        Account {},
}

fn main() {
    dioxus::launch(App);
}

/// The root component, mounted on both the client and the server.
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
