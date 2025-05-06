use super::deps::{components::prelude::*, *};

#[component]
pub fn GoBack() -> Element {
    rsx! {
        BackButton {}
        Outlet::<Route> {}
    }
}
