use dioxus::prelude::*;

/// Expense entry and history, backed by the `finance` crate's ledger.
#[component]
pub fn Expenses() -> Element {
    rsx! {
        h1 { "Expenses" }
        p { "Record an expense and browse the ledger." }
    }
}
