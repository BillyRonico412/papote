use maud::{Markup, html};

use crate::utils::{Htmx, button, page};

pub async fn handler(htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            div class="flex flex-col justify-center-safe h-full" {
                p class="title text-4xl text-center" {
                    "Le principe du jeu"
                }
                p class="title text-lg mt-5" {
                    "Comment ça marche ?"
                }
                p class="mt-5" {
                    "Papote est né d'un constat simple :"
                    br;
                    b { "les meilleures discussions ne tombent pas du ciel, elles se créent" }
                    "."
                }
                p class="mt-5" {
                    "Que vous ayez envie d'un dîner tout doux ou d'une soirée survoltée, l'application s'adapte à votre groupe et à votre humeur du moment."
                }
                p class="title text-lg mt-5" {
                    "En 3 étapes, c'est parti :"
                }
                ol class="list-decimal list-inside" {
                    li class="ml-4" {
                        "Choisissez votre équipe :"
                        br;
                        "En amoureux ou entre amis, donnez le ton de votre soirée"
                    }
                    li class="ml-4" {
                        "Ajustez vos curseurs :"
                        br;
                        "Intimité, piquant, humour, proximité, énergie... Réglez les curseurs selon la vibe du moment pour façonner la question qui vous ressemble"
                    }
                    li class="ml-4" {
                        "Laissez la magie opérer :"
                        br;
                        "Piochez, lisez, et laissez place aux vrais échanges, aux rires et aux confidences"
                    }
                }
                p class="mt-5" {
                    "Moins de filtres, plus de connexions."
                }
                div class="mt-15 self-center" {
                    a
                        hx-get="/choices"
                        hx-push-url="true"
                        hx-target="#main"
                    {
                        (button(html!("Jouer")))
                    }
                }
            }
        },
    )
}
