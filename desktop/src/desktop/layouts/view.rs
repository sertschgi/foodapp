use crate::prelude::{components::prelude::*, *};

#[component]
pub fn View() -> Element {
    use ui::components::prelude::*;
    rsx! {
        View {
            Outlet::<Route> {}
        }
    }
}
