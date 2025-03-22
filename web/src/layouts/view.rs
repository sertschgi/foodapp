use crate::*;

#[component]
pub fn View() -> Element {
    use ui::components::prelude::*;
    rsx! {
        View {
            Outlet::<Route> {}
        }
    }
}
