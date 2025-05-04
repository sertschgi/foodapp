use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const ACCOUNT_CSS: Asset = asset!("/assets/styling/pages/account.css");

#[component]
pub fn Account() -> Element {
    let mut session =
        use_synced_storage::<LocalStorage, Option<LoginSession>>("session".into(), || {
            Option::<LoginSession>::None
        });
    let mut toggle = use_signal(|| true);
    let loggedin = use_memo(move || session().is_some());
    let mut user_info = use_signal(|| Option::<UserInfo>::None);
    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_CSS }


        div {
            id: "Account",
            if loggedin() {
                section {
                    class: "wrapper",
                    onvisible: move |_| async move {
                        user_info.set(get_user_info(session().unwrap()).await.ok());
                    },
                    if let Some(UserInfo {username, email, created_at, ..}) = user_info() {
                        h2 {"Hi {username}, du bist eingeloggt!"}
                        p { "email: {email}"}
                        p { "registriert seit: {created_at}"}
                    }
                    button {
                        onclick: move |_| async move {
                            if logout(session.unwrap()).await.is_ok() {
                                session.set(None);
                            }
                        },
                        LogoutIcon {}
                        "ausloggen"
                    }
                }
            } else {
                if toggle() {
                    form {
                        onsubmit: move |e| async move {
                            debug!("{:?}", e.values());
                            let values = e.values();
                            let ident = values.get("ident").unwrap().as_value();
                            let password = values.get("password").unwrap().as_value();
                            let request = LoginRequest {
                                ident,
                                password,
                            };
                            debug!("{:?}", request);
                            session.set(login(request).await.ok())
                         },
                        label {
                            "Benutzername/E-Mail:"
                            br {}
                            input {
                                r#type: "text",
                                name: "ident"
                            }
                        }
                        br {}
                        label {
                            "Passwort:"
                            br {}
                            input {
                                r#type: "password",
                                name: "password"
                            }
                        }
                        br {}
                        button {
                            "Einloggen"
                        }
                    }
                    p {
                        "Noch kein Konto?"
                        a {
                            cursor: "pointer",
                            onclick: move |_| { toggle.set(false); },
                            "Registriere dich!"
                        }
                    }
                } else {
                    form {
                        onsubmit: move |e| async move {
                            debug!("{:?}", e.values());
                            let values = e.values();
                            let email = values.get("email").unwrap().as_value();
                            let username = values.get("username").unwrap().as_value();
                            let password = values.get("password").unwrap().as_value();
                            let request = RegisterRequest {
                                email,
                                username,
                                password,
                            };
                            debug!("{:?}", request);
                            debug!("{:?}", register(request).await);
                        },
                        label {
                            "E-Mail:"
                            br {}
                            input {
                                r#type: "email",
                                name: "email"
                            }
                        }
                        br {}
                        label {
                            "Benutzername:"
                            br {}
                            input {
                                r#type: "text",
                                name: "username"
                            }
                        }
                        br {}
                        label {
                            "Passwort:"
                            br {}
                            input {
                                r#type: "text",
                                name: "password"
                            }
                        }
                        br {}
                        button {
                            "Registrieren"
                        }
                    }
                    p {
                        "Schon ein Konto?"
                        a {
                            cursor: "pointer",
                            onclick: move |_| { toggle.set(true); },
                            "Einloggen!"
                        }
                    }
                }
            }
        }
    }
}
