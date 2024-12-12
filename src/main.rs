mod backend;
mod shared;

use dioxus::prelude::*;
use shared::types::{ActivityLevel, Age};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/calculator")]
    Calculator {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Calculator() -> Element {
    let mut selected = use_signal(|| false);
    rsx! {
        div {
            id: "calculator",

            // Content
            h1 { "This is a calculator" }
            EnumInputComponent {}
        }
    }
}

#[component]
fn EnumInputComponent() -> Element {
    let mut age = use_signal(|| Age::Puppy);
    let mut activity_level = use_signal(|| ActivityLevel::Sedentary);
    let mut weight_kg = use_signal(|| 0);

    rsx! {
        div {
            div {
                "First Enum: {age()}",
                div {
                    {Age::iter().map(|variant| rsx!(
                        button {
                            class: if age() == variant { "rounded-md bg-blue-600 py-2 px-4 border border-transparent text-center text-sm text-white transition-all shadow-md hover:shadow-lg focus:bg-blue-700 focus:shadow-none active:bg-blue-700 hover:bg-blue-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none ml-2" } else { "red" },
                            onclick: move |_| age.set(variant),
                            "{variant}"
                        }
                    ))}
                },
            },
            "Second Enum: {activity_level()}",
            button {
                onclick: move |_| activity_level.set(ActivityLevel::Moderate),
                "Change Second Enum"
            },
            "Number: {weight_kg()}",
            button {
                onclick: move |_| weight_kg.set(weight_kg() + 1),
                "Increment Number"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Calculator {},
                "Nutrient Calculator"
            }
        }

        Outlet::<Route> {}
    }
}
