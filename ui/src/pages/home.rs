use crate::prelude::{components::prelude::*, *};

const HOME_CSS: Asset = asset!("/assets/styling/pages/home.css");

#[component]
pub fn Home(
    #[props(into)] account_route: NavigationTarget,
    #[props(into)] settings_route: NavigationTarget,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div {
            id: "Favourite",
             PageHeader {
                Link {
                    to: settings_route,
                    SettingsIcon {}
                }
                Link {
                    to: account_route,
                    AccountIcon {}
                }
             }
        }
    }
}
