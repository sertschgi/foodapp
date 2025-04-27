use crate::prelude::{components::prelude::*, *};

const ACCOUNT_CSS: Asset = asset!("/assets/styling/pages/account.css");

#[component]
pub fn Account() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_CSS }

        div {
            id: "Favourite",
            AccountIcon {}
            form {
                label {
                    "Benutzername:"
                    input {

                    }
                }
                label {
                    "Passwort:"
                    input {

                    }
                }
                button {
                    "Einloggen"
                }
            }
        }
    }
}
