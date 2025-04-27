use crate::prelude::{components::prelude::*, *};

#[component]
pub fn MobileTopBar() -> Element {
    rsx! {
        TopBar {}
        Outlet::<Route> {}
    }
}
