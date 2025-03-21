use dioxus::prelude::*;

const NAVBAR_BUTTON_CSS: Asset = asset!("/assets/styling/components/navbar_button.css");

#[component]
pub fn NavbarButton(
    children: Element,
    class: Option<String>,
    #[props(into)] to: NavigationTarget,
) -> Element {
    document::eval(&format!(
        r#"
navigation.addEventListener("navigate", (e) => {{
const currentUrl = e.destination.url;
const links = document.querySelectorAll('.NavbarButton');

links.forEach(link => {{
  console.log(link.href, currentUrl);
  if (link.href === currentUrl) {{
    link.classList.add('active');

  }}
  else {{
  link.classList.remove('active');
  }}
}});
}})
        "#
    ));
    let class_unw = class.unwrap_or_default();
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_BUTTON_CSS }
        Link {
            class: "NavbarButton {class_unw}",
            to: to,
            {children}
        }
    }
}
