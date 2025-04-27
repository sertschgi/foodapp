use crate::prelude::{components::prelude::*, *};

#[component]
pub fn GoBack() -> Element {
    rsx! {
        BackButton {}
        Outlet::<Route> {}
    }
}
