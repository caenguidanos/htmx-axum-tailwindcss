use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::AppState;

pub async fn page(State(state): State<AppState>) -> impl IntoResponse {
    match state
        .template
        .render("views/debug/index.html", &tera::Context::new())
    {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
    }
}

pub mod api {
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::{Html, IntoResponse};
    use sysinfo::SystemExt;

    use crate::AppState;

    pub async fn timestamp(State(state): State<AppState>) -> impl IntoResponse {
        let mut ctx = tera::Context::new();
        ctx.insert("iso", &chrono::Utc::now().to_rfc3339());

        match state.template.render("components/timestamp.html", &ctx) {
            Ok(html) => (StatusCode::OK, Html(html)),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
        }
    }

    pub async fn sys_info(State(state): State<AppState>) -> impl IntoResponse {
        let mut sys = state.sys.lock().unwrap();

        sys.refresh_all();

        let total_mem = (sys.total_memory() / 1024 / 1024).to_string();
        let used_mem = (sys.used_memory() / 1024 / 1024).to_string();

        let mut ctx = tera::Context::new();
        ctx.insert("cpus", &sys.cpus().len());
        ctx.insert("total_mem", &total_mem);
        ctx.insert("used_mem", &used_mem);

        match state.template.render("components/sys_info.html", &ctx) {
            Ok(html) => (StatusCode::OK, Html(html)),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Html(err.to_string())),
        }
    }
}
