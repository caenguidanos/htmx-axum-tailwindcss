use std::sync::{Arc, Mutex};

use axum::{response::Html, routing::get, Router};
use sysinfo::{System, SystemExt};

mod components;

#[tokio::main]
async fn main() {
    let sys = Arc::new(Mutex::new(System::new_all()));
    let cloned_sys = Arc::clone(&sys);

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                let content =
                    components::compose_base_route_with(include_str!("./routes/index.html"));

                Html(content)
            }),
        )
        .route(
            "/debug",
            get(|| async {
                let content =
                    components::compose_base_route_with(include_str!("./routes/debug/index.html"));

                Html(content)
            }),
        )
        .nest(
            "/api",
            Router::new()
                .route("/call", get(|| async { Html(components::HELLO) }))
                .route(
                    "/timestamp",
                    get(|| async {
                        let now = chrono::Utc::now();

                        Html(components::TIMESTAMP.replace("{value}", &now.to_rfc3339()))
                    }),
                )
                .route(
                    "/sysinfo",
                    get(|| async move {
                        let mut sys = cloned_sys.lock().unwrap();

                        sys.refresh_all();

                        let cpus = sys.cpus().len().to_string();
                        let total_mem = (sys.total_memory() / 1024 / 1024).to_string();
                        let used_mem = (sys.used_memory() / 1024 / 1024).to_string();

                        Html(
                            components::SYSINFO
                                .replace("{cpus}", &cpus)
                                .replace("{total_mem}", &total_mem)
                                .replace("{used_mem}", &used_mem),
                        )
                    }),
                ),
        )
        .nest_service("/public", tower_http::services::ServeDir::new("public"));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
