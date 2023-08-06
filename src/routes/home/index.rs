use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::AppState;

pub async fn page(State(state): State<AppState>) -> impl IntoResponse {
    match state
        .template
        .render("views/index.html", &tera::Context::new())
    {
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
