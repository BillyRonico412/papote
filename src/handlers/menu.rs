use maud::{Markup, html};

use crate::utils::{Htmx, page};

pub async fn handler(htmx: Htmx) -> Markup {
    page(
        htmx,
        html! {
            div class="w-full h-full flex flex-col items-center justify-center gap-15" {
                a
                    hx-get="/principe"
                    hx-push-url="true"
                    hx-target="#main"
                    class="title text-lg"
                {
                    "Le principe du jeu"
                }
                a
                    hx-get="/a-propos"
                    hx-push-url="true"
                    hx-target="#main"
                    class="title text-lg"
                {
                    "À propos"
                }
            }
        },
    )
}
