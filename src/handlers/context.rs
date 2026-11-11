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
            form
                class="w-full h-full flex flex-col items-center gap-6"
                hx-get="/questions"
                hx-vals=(format!("{{\"mode\": \"{}\"}}", mode.to_string().to_lowercase()))
                hx-target="#main"
                hx-push-url="true"
            {
                label class="title text-2xl" {
                    "Votre ambiance"
                }
                textarea
                    name="context"
                    class="w-full border-2 border-white rounded-lg flex-1 p-6"
                    placeholder="Décrivez l'ambiance de la soirée : qui est là, le lieu, l'humeur du moment, les sujets à aborder ou à éviter… \nEx : « Apéro détendu entre vieux amis, on veut rire et se remémorer des souvenirs. »"
                {}
                // button class="flex items-center gap-2" {
                //     "Options avancées"
                //     img src="/static/arrow-right.svg" class="";
                // }
                (&button(html!("Générer")))
                button type="button" class="uppercase font-bold flex items-center gap-2" {
                    img src="/static/arrow-rotate.svg" class="";
                    "Réinitialiser"
                }
            }
        },
    )
}
