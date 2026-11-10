use axum::extract::Path;
use maud::{Markup, html};

use crate::{
    logic::Mode,
    utils::{Htmx, button, page},
};

pub async fn handler(Path(mode): Path<Mode>, htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            form class="w-full h-full flex flex-col items-center gap-6" {
                label class="title text-2xl" {
                    "Votre ambiance"
                }
                textarea
                    class="w-full border-2 border-white rounded-lg flex-1 p-6"
                    placeholder="Je suis au restaurant avec des amis d'enfance"
                {}
                button class="flex items-center gap-2" {
                    "Options avancées"
                    img src="/static/arrow-right.svg" class="";
                }
                (&button(html!("Générer")))
                button class="uppercase font-bold flex items-center gap-2" {
                    img src="/static/arrow-rotate.svg" class="";
                    "Réinitialiser"
                }
            }
        },
    )
}
