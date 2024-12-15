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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero" }
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
    rsx! {
        div { id: "calculator",

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
        div { class: "relative right-0 p-4",
            ul { class: "w-5/12 mx-auto relative flex flex-wrap px-2 py-2 list-none rounded-md bg-gray-200 dark:bg-gray-800 shadow-md",
                div { class: "calculator-item-header", "Select Age" }
                li { class: "z-30 flex-auto text-center m-1",
                    {Age::iter().map(|variant| rsx! {
                        button {
                            class: "selectable-button",
                            aria_pressed: if age() == variant { false } else { true },
                            onclick: move |_| age.set(variant),
                            "{variant}"
                        }
                    })}
                }
                div { class: "calculator-item-header", "Select Activity Level" }
                li { class: "z-30 flex-auto text-center m-1",
                    {ActivityLevel::iter().map(|variant| rsx! {
                        button {
                            class: "selectable-button",
                            aria_pressed: if activity_level() == variant { false } else { true },
                            onclick: move |_| activity_level.set(variant),
                            "{variant}"
                        }
                    })}
                }
                div { class: "calculator-item-header", "Select Weight" }
                li { class: "z-30 flex-auto text-center m-1",
                    div { class: "flex justify-center text-center m-1",
                        div { class: "relative",
                            button {
                                r#type: "button",
                                class: "absolute right-9 top-1 rounded bg-slate-800 p-1.5 border border-transparent text-center text-sm text-white transition-all shadow-sm hover:shadow focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
                                id: "weightIncreaseButton",
                                MinusIcon {}
                            }
                            input {
                                r#type: "number",
                                value: "0",
                                class: "w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md pl-3 pr-20 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow appearance-none [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                id: "weightInput",
                            }
                            button {
                                r#type: "button",
                                class: "absolute right-1 top-1 rounded bg-slate-800 p-1.5 border border-transparent text-center text-sm text-white transition-all shadow-sm hover:shadow focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
                                id: "weightDecreaseButton",
                                PlusIcon {}
                            }
                        }
                    }
                }
            }
            "Number: {weight_kg()}"
            button { onclick: move |_| weight_kg.set(weight_kg() + 1), "Increment Number" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Calculator {}, "Nutrient Calculator" }
        }

        Outlet::<Route> {}
    }
}

fn PlusIcon() -> Element {
    rsx!(
        svg { view_box: "0 0 16 16", fill: "currentColor", class: "w-4 h-4",
            path { d: "M8.75 3.75a.75.75 0 0 0-1.5 0v3.5h-3.5a.75.75 0 0 0 0 1.5h3.5v3.5a.75.75 0 0 0 1.5 0v-3.5h3.5a.75.75 0 0 0 0-1.5h-3.5v-3.5Z" }
        }
    )
}

fn MinusIcon() -> Element {
    rsx!(
        svg { view_box: "0 0 16 16", fill: "currentColor", class: "w-4 h-4",
            path { d: "M3.75 7.25a.75.75 0 0 0 0 1.5h8.5a.75.75 0 0 0 0-1.5h-8.5Z" }
        }
    )
}
