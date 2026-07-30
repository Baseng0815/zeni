use dioxus::prelude::*;

/// The overview: recent spending alongside stock that needs attention.
#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Overview" }
        p { "Recent spending and stock needing attention." }
    }
}
