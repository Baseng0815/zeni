use dioxus::prelude::*;

/// User account: registration, login, and settings such as default currency.
#[component]
pub fn Account() -> Element {
    rsx! {
        h1 { "Account" }
        p { "Sign in, register, and manage settings." }
    }
}
