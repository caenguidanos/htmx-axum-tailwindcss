use std::sync::{Arc, Mutex};

use axum::routing::get;
use axum::Router;
use sysinfo::{System, SystemExt};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod routes;

#[derive(Clone)]
pub struct AppState {
    sys: Arc<Mutex<System>>,
    template: tera::Tera,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().compact().pretty().init();

    routes::dist::build_static().await;
    routes::dist::compress_all().await;

    let state = AppState {
        sys: Arc::new(Mutex::new(System::new_all())),
        template: tera::Tera::new("src/templates/**/*.html").unwrap(),
    };

    let app = Router::new()
        .route("/", get(routes::home::page))
        .route("/debug", get(routes::debug::page))
        .nest(
            "/api",
            Router::new()
                .route("/home/hello", get(routes::home::api::hello))
                .route("/debug/timestamp", get(routes::debug::api::timestamp))
                .route("/debug/sys_info", get(routes::debug::api::sys_info)),
        )
        .nest(
            "/dist",
            Router::new().route("/*file", get(routes::dist::file_handler)),
        )
        .nest_service("/public", tower_http::services::ServeDir::new("public"))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
