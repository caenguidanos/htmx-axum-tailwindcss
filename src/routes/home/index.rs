use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::AppState;

#[derive(serde::Serialize)]
pub struct NavbarLink {
    pub name: String,
    pub href: String,
}

pub async fn page(State(state): State<AppState>) -> impl IntoResponse {
    let mut ctx = tera::Context::new();
    ctx.insert(
        "links",
        &vec![
            NavbarLink {
                name: String::from("Home"),
                href: String::from("/"),
            },
            NavbarLink {
                name: String::from("Debug"),
                href: String::from("/debug"),
            },
        ],
    );

    match state.template.render("views/index.html", &ctx) {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
    }
}

pub mod api {
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::{Html, IntoResponse};

    use crate::AppState;

    pub async fn hello(State(state): State<AppState>) -> impl IntoResponse {
        match state
            .template
            .render("components/hello.html", &tera::Context::new())
        {
            Ok(html) => (StatusCode::OK, Html(html)),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
        }
    }
}
