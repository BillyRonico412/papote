use maud::{Markup, html};

use crate::utils::{Htmx, page};

pub async fn handler(htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            div class="flex flex-col items-center justify-center-safe text-center text-balance h-full" {
                p class="title text-4xl" {
                    "C'est quoi ?"
                }
                p class="title text-lg mt-5" {
                    "Bienvenue sur Papote !"
                    br;
                    "Vos conversations sur-mesure"
                }
                p class="mt-5" {
                    "À deux ou entre amis, choisissez votre groupe, ajustez vos curseurset laissez l'application générer la question parfaite pour rythmer votre soirée."
                    br;
                    "Des échanges taillés sur-mesure pour créerun vrai moment de partage"
                }
                div class="text mt-20" {
                    a
                        class="btn"
                        hx-get="/choices"
                        hx-push-url="true"
                        hx-target="#main"
                    {
                        "C'est parti"
                    }
                }
            }
        },
    )
}
