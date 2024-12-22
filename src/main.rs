mod backend;
mod shared;

use backend::nutrients::{FromValue, Intake, Kilogram, MetabolicBodyWeight, NewRecommendedIntake};
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
            EnumInputComponent {}
        }
    }
}

#[component]
fn EnumInputComponent() -> Element {
    let mut age = use_signal(|| Age::default());
    let mut activity_level = use_signal(|| ActivityLevel::default());
    let mut weight_kg: Signal<u32> = use_signal(|| 0);
    let mut intake = use_signal(|| Intake::default());

    use_effect(move || {
        let metabolic_bw = MetabolicBodyWeight::new(Kilogram::from_value(weight_kg() as f32));
        intake.set(Intake::new_recommended_intake(
            &metabolic_bw,
            activity_level(),
        ));
    });

    rsx! {
        div { class: "container w-[70%] mx-auto px-4 py-8",
            // Calculator inputs section
            ul { class: "w-[70%] mx-auto flex flex-wrap p-2 rounded-md bg-gray-200 dark:bg-gray-800 shadow-md",
                // Age and Activity Level sections use the same pattern
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

                // Weight input section
                div { class: "calculator-item-header", "Select Weight (kg)" }
                li { class: "z-30 flex-auto text-center m-1",
                    div { class: "flex justify-center text-center m-1",
                        div { class: "relative",
                            button {
                                r#type: "button",
                                class: "absolute right-9 top-1 rounded bg-slate-800 p-1.5 border border-transparent text-center text-sm text-white transition-all shadow-sm hover:shadow focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
                                id: "weightDecreaseButton",
                                onclick: move |_| weight_kg.set(weight_kg().saturating_sub(1)),
                                MinusIcon {}
                            }
                            input {
                                r#type: "number",
                                value: "{weight_kg()}",
                                min: "0",
                                class: "w-min-0 bg-transparent placeholder:text-slate-400 text-white text-sm border border-slate-200 rounded-md pl-3 pr-20 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow appearance-none [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                id: "weightInput",
                            }
                            button {
                                r#type: "button",
                                class: "absolute right-1 top-1 rounded bg-slate-800 p-1.5 border border-transparent text-center text-sm text-white transition-all shadow-sm hover:shadow focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none",
                                id: "weightIncreaseButton",
                                onclick: move |_| weight_kg.set(weight_kg().saturating_add(1)),
                                PlusIcon {}
                            }
                        }
                    }
                }
            }

            // Results table section
            div { class: "mt-4",
                h1 { class: "text-2xl font-bold text-center mb-6", "Nutrient Intake" }
                table { class: "w-[80%] mx-auto bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden",
                    thead { class: "bg-gray-100 dark:bg-gray-700",
                        tr {
                            th { class: "table-header w-1/2", "Category" }
                            th { class: "table-header w-1/2 text-right", "Value" }
                        }
                    }
                    tbody { class: "divide-y divide-gray-200 dark:divide-gray-700",
                        // Main nutrients section
                        tr { class: "table-row-alt",
                            td { class: "table-cell-label w-1/2", "Daily Calories" }
                            td { class: "table-cell-value w-1/2 text-right", "{intake().daily_kcal}" }
                        }
                        tr { class: "table-row-alt",
                            td { class: "table-cell-label w-1/2", "Protein" }
                            td { class: "table-cell-value w-1/2 text-right", "{intake().nutrients.protein}" }
                        }
                        tr { class: "table-row-alt",
                            td { class: "table-cell-label w-1/2", "Fat" }
                            td { class: "table-cell-value w-1/2 text-right", "{intake().nutrients.fat}" }
                        }

                        // Amino Acids section
                        tr {
                            td { class: "table-section-header", colspan: "2", "Amino Acids" }
                        }
                        {
                            let current_intake = intake();
                            rsx!({
                                (&current_intake.nutrients.amino_acids).into_iter().map(|(name, value)| rsx!(
                                    tr { class: "table-row-alt",
                                        td { class: "table-cell-label w-1/2", "{name}" }
                                        td { class: "table-cell-value w-1/2 text-right", "{value}" }
                                    }
                                ))
                            })
                        }

                        // Fatty Acids section
                        tr {
                            td { class: "table-section-header", colspan: "2", "Fatty Acids" }
                        }
                        {
                            let current_intake = intake();
                            rsx!({
                                (&current_intake.nutrients.fatty_acids).into_iter().map(|(name, value)| rsx!(
                                    tr { 
                                        td { class: "table-cell-label w-1/2", "{name}" }
                                        td { class: "table-cell-value w-1/2 text-right", "{value}" }
                                    }
                                ))
                            })
                        }

                        // Minerals section
                        tr {
                            td { class: "table-section-header", colspan: "2", "Minerals" }
                        }
                        {
                            let current_intake = intake();
                            rsx!({
                                (&current_intake.nutrients.minerals).into_iter().map(|(name, value)| rsx!(
                                    tr {class: "table-row-alt",
                                        td { class: "table-cell-label w-1/2", "{name}" }
                                        td { class: "table-cell-value w-1/2 text-right", "{value}" }
                                    }
                                ))
                            })
                        }

                        // Vitamins section
                        tr {
                            td { class: "table-section-header", colspan: "2", "Vitamins" }
                        }
                        {
                            let current_intake = intake();
                            rsx!({
                                (current_intake.nutrients.vitamins).into_iter().map(|(name, value)| rsx!(
                                    tr { class: "table-row-alt",
                                        td { class: "table-cell-label w-1/2", "{name}" }
                                        td { class: "table-cell-value w-1/2 text-right", "{value}" }
                                    }
                                ))
                            })
                        }
                    }
                }
            }
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
