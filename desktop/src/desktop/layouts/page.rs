use super::deps::{components::prelude::*, *};

#[component]
pub fn Page() -> Element {
    use ui::components::prelude::*;
    rsx! {
        Page {
            Outlet::<Route> {}
        }
    }
}
