use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::AppState;

pub async fn debug(State(state): State<AppState>) -> impl IntoResponse {
    match state
        .template
        .render("routes/debug/index.html", &tera::Context::new())
    {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
    }
}
