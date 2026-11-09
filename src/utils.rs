use axum::{extract::FromRequestParts, http::StatusCode, response::{IntoResponse, Response}};
use maud::{DOCTYPE, Markup, html};

pub struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR, 
            layout(html! {
                main class="h-full w-full flex items-center-safe justify-center-safe" {
                    div role="alert" class="alert alert-error alert-soft alert-vertical" {
                        h3 class="font-bold" {
                            "Erreur lors du chargement de l'url. Soit l'url est incorect, soit le site est en panne !"
                        }
                        p class="text-xs" {
                            (&self.0.to_string())
                        }
                    }
                }
            })
        ).into_response()
    }
}

pub struct Htmx(bool);

impl<S> FromRequestParts<S> for Htmx {
    type Rejection = ();

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            Ok(Self(parts.headers.get("HX-Request").is_some()))
        }
    }
}


pub fn layout(children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { "Urban Soccer Video" }
            meta name="viewport" content="width=device-width, initial-scale=1";
            link href="/static/output.css" rel="stylesheet";
            script src="/static/htmx.js" {};
            script src="/static/alpine.js" {};
        }
        body class="w-screen h-screen overflow-hidden flex flex-col" {
            div
                x-data="{ ready: false }"
                x-init="setTimeout(() => { ready = true }, 1000)"
            {
                div
                    class="flex justify-center-items-center w-dvw h-dvh relative"
                    x-show="!ready"
                    x-transition
                {
                    img src="/static/logo.svg" class="w-full";
                    p
                        class="absolute bottom-4 left-1/2 -translate-x-1/2 text-center text-base italic"
                    {
                        "Pensé et créé par" br; "Ronico Billy & Rachel Bastié"
                    }
                }
                div
                    class="flex flex-col gap-4 w-dvw h-dvh relative"
                    x-show="ready" x-transition
                {
                    header class="flex items-center justify-beetween px-4 py-2" {
                        img src="/static/mini-logo.svg" class="h-12";
                        img src="/static/burger.svg" class="h-6 ml-auto";
                    }
                    main class="flex-1 px-4 py-8 overflow-y-auto" id="main" {
                        (&children)
                    }
                }
            }
        }
    }
}

pub fn page(
    htmx: Htmx,    
    children: Markup
) -> Markup {
    if htmx.0 {
        children
    } else {
        layout(children)
    }
}

pub fn button(
        children: Markup
    ) -> Markup {
    html! {
        button class="px-6 py-2 uppercase bg-white text-primary rounded-lg font-bold" {
            (&children)
        }
    }
}