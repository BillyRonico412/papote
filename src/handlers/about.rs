use maud::{Markup, html};

use crate::utils::{Htmx, button, page};

pub async fn handler(htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            div class="flex flex-col items-center justify-center-safe text-center text-balance h-full" {
                p class="title text-4xl" {
                    "À propos"
                }
                p class="title text-lg mt-5" {
                    "Derrière Papote..."
                }
                p class="mt-5" {
                    "Salut !"
                    br;
                    b { "Nous, c'est Rachel et Billy." }
                    br;
                    "Derrière les écrans et les lignes de code de Papote, il y a avant tout un couple "
                    b { "passionné par les belles dynamiques humaines" }
                    "."
                }
                p class="mt-5" {
                    "On a créé ce jeu avec une idée fixe : remettre l'humain et la discussion au centre de nos soirées, pour mieux se découvrir et échanger."
                }
                p class="mt-5" {
                    b { "L'objectif" }
                    " de Papote est simple : offrir un "
                    b { "espace de liberté pour se redécouvrir, rigoler" }
                    " un bon coup et "
                    b { "créer du vrai lien" }
                    ", que l'on soit deux sous un plaid ou dix autour d'une grande table."
                }
                p class="mt-5" {
                    "On a conçu chaque détail pour que "
                    b { "chaque partie soit unique" }
                    ", tout comme vous."
                }
                p class="title text-lg mt-5" {
                    "Alors... Prêts à papoter ?"
                }
                div class="mt-15" {
                    a
                        hx-get="/choices"
                        hx-push-url="true"
                        hx-target="#main"
                    {
                        (button(html!("C'est parti")))
                    }
                }
            }
        },
    )
}
