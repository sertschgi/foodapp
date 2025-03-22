use crate::*;

const NAVBAR_BUTTON_CSS: Asset = asset!("/assets/styling/components/navbar_button.css");

#[component]
pub fn NavbarButton(
    children: Element,
    class: Option<String>,
    #[props(into)] to: NavigationTarget,
) -> Element {
    let mut active = use_signal(String::new);
    let unw_class = class.unwrap_or_default();
    let classes = use_memo(move || {
        debug!("{active}");
        format!("{} {}", unw_class, active())
    });
    let to_signal = use_signal(|| to);

    use_effect(move || {
        let current_url = router().full_route_string();
        let href = match to_signal() {
            NavigationTarget::Internal(url) => url.clone(),
            NavigationTarget::External(route) => route.clone(),
        };
        debug!("{current_url}, {href}");
        if current_url == href {
            active.set("active".into());
            debug!("{classes}");
        } else {
            active.set("".into());
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_BUTTON_CSS }
        Link {
            class: "NavbarButton {classes}",
            to: to_signal(),
            div {
                {children}
            }
        }
    }
}
