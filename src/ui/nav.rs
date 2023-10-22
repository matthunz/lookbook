use crate::{prefixed_route::PrefixedRoute, ui::pane::HorizontalPane, Route, CONTEXT};
use dioxus::prelude::*;
use dioxus_material::use_theme;
use dioxus_router::prelude::*;

#[component]
pub fn Nav(cx: Scope) -> Element {
    let elements = CONTEXT.try_with(|cx| cx.borrow().clone()).unwrap();

    cx.render(rsx! {
        div {
            width: "100vw",
            height: "100vh",
            display: "flex",
            flex_direction: "row",
            font_family: "sans-serif",
            margin: 0,
            padding: 0,
            HorizontalPane {
                left: render!(
                    div { flex : 1, display : "flex", flex_direction : "column", gap : "10px", width :
                    "200px", margin : 0, padding : "10px 5px", font_size : "14px", background :
                    "#eeeeee", elements.into_iter().map(move | (name, _) | { render!(NavItem { route :
                    Route::ComponentScreen { name : name.to_string(), }, label : "{name}" }) }) }
                ),
                right: render!(Outlet::< PrefixedRoute > {})
            }
        }
    })
}

#[component]
fn NavItem<'a>(cx: Scope<'a>, route: Route, label: &'a str) -> Element<'a> {
    let navigator = use_navigator(cx);
    let current_route: Option<PrefixedRoute> = use_route(cx);
    let theme = use_theme(cx);

    let prefixed_route = PrefixedRoute(route.clone());
    let is_selected = current_route.as_ref() == Some(&prefixed_route);

    render!(
        div {
            padding: "10px 15px",
            border_radius: &*theme.border_radius,
            cursor: "pointer",
            background: if is_selected { &theme.secondary_container_color } else { "" },
            onclick: |_| {
                navigator
                    .push(
                        PrefixedRoute(Route::ComponentScreen {
                            name: label.to_string(),
                        }),
                    );
            },
            "{label}"
        }
    )
}
