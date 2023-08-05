use std::sync::{Arc, Mutex};

use axum::routing::get;
use axum::Router;
use sysinfo::{System, SystemExt};
use tower_http::trace::TraceLayer;

mod api;
mod routes;

#[derive(Clone)]
pub struct AppState {
    sys: Arc<Mutex<System>>,
    template: tera::Tera,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let state = AppState {
        sys: Arc::new(Mutex::new(System::new_all())),
        template: tera::Tera::new("src/**/*.html").unwrap(),
    };

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(routes::home))
        .route("/debug", get(routes::debug))
        .nest(
            "/api",
            Router::new()
                .route("/call", get(api::call))
                .route("/timestamp", get(api::timestamp))
                .route("/sysinfo", get(api::sysinfo))
                .with_state(state.clone()),
        )
        .nest_service("/public", tower_http::services::ServeDir::new("public"))
        .nest_service("/dist", tower_http::services::ServeDir::new("dist"))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
