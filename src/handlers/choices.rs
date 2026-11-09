use maud::{Markup, html};

use crate::utils::{Htmx, button, page};

pub async fn handler(htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            div class="h-full flex flex-col items-center justify-around" {
                div class="flex flex-col items-center" {
                    p class="text-4xl uppercase title" {
                        "En couple"
                    }
                    p class="mt-5" {
                        "Un tête-à-tête où se découvrir"
                    }
                    div class="mt-15" {
                        (&button(html!("Choisir")))
                    }
                }

                hr class="h-px bg-white w-full";

                div class="flex flex-col items-center" {
                    p class="text-4xl uppercase title" {
                        "En couple"
                    }
                    p class="mt-5" {
                        "Un tête-à-tête où se découvrir"
                    }
                    div class="mt-15" {
                        (&button(html!("Choisir")))
                    }
                }
            }
        },
    )
}
