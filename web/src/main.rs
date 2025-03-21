use dioxus::prelude::*;

mod app;
mod layouts;
mod route;

fn main() {
    dioxus::launch(app::App);
}
