use maud::{Markup, html};

use crate::utils::{Htmx, page};

pub async fn handler(htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            div class="w-full h-full flex flex-col items-center justify-around" {
                div class="flex flex-col items-center" {
                    p class="text-4xl uppercase title" {
                        "En couple"
                    }
                    p class="mt-5" {
                        "Un tête-à-tête où se découvrir"
                    }
                    div class="mt-15" {
                        a
                            class="btn"
                            hx-get="/context/couple"
                            hx-push-url="true"
                            hx-target="#main"
                        {
                            "Choisir"
                        }
                    }
                }

                hr class="h-px bg-white w-full";

                div class="flex flex-col items-center" {
                    p class="text-4xl uppercase title" {
                        "Entre amis"
                    }
                    p class="mt-5" {
                        "Pour lancer la bonne énergie du soir"
                    }
                    div class="mt-15" {
                        a
                            class="btn"
                            hx-get="/context/amis"
                            hx-push-url="true"
                            hx-target="#main"
                        {
                            "Choisir"
                        }
                    }
                }
            }
        },
    )
}
