use dioxus::prelude::*;

use crate::Route;

/// The layout wrapping every route: shared chrome plus an outlet for the
/// active route's view.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Overview" }
            Link { to: Route::Expenses {}, "Expenses" }
            Link { to: Route::Stock {}, "Stock" }
            Link { to: Route::Account {}, "Account" }
        }

        main { id: "content", Outlet::<Route> {} }
    }
}
