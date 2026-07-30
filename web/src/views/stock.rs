use dioxus::prelude::*;

/// On-hand stock, expiries, and the shopping list, backed by the `inventory`
/// crate.
///
/// Named `Stock` rather than `Inventory` so the module never shadows the
/// `inventory` crate at a use site.
#[component]
pub fn Stock() -> Element {
    rsx! {
        h1 { "Stock" }
        p { "What is on hand, what expires soon, what to buy." }
    }
}
