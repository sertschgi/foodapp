use super::deps::{components::prelude::*, *};
use change_case::pascal_case;
use percent_encoding::percent_decode_str;

#[component]
pub fn MobileTopBar() -> Element {
    let route_str = router().full_route_string();
    let encoded = route_str.split("/").last().unwrap_or_default();
    let unfmt_name: String = percent_decode_str(encoded).decode_utf8_lossy().into_owned();
    let title = pascal_case(&unfmt_name);
    rsx! {
        TopBar {
            title
        }
        Outlet::<Route> {}
    }
}
