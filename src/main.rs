use std::sync::{Arc, Mutex};

use axum::routing::get;
use axum::Router;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
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

    process_dist().await;

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
                .route("/debug/sys_info", get(routes::debug::api::sys_info))
                .route("/common/navbar", get(routes::api::navbar)),
        )
        .nest_service("/dist", tower_http::services::ServeDir::new("dist"))
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

async fn process_dist() {
    if tokio::fs::try_exists("./dist").await.unwrap() {
        tokio::fs::remove_dir_all("./dist").await.unwrap();
    }

    process_dist_css().await;
    process_dist_js().await;
}

async fn process_dist_js() {
    tracing::info!("htmx build");

    tokio::fs::create_dir_all("./dist/js").await.unwrap();

    let htmx = reqwest::get("https://unpkg.com/htmx.org@1.9.4/dist/htmx.min.js")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    tokio::fs::write("./dist/js/htmx.min.js", htmx)
        .await
        .unwrap();
}

async fn process_dist_css() {
    tracing::info!("css build");

    tokio::fs::create_dir_all("./dist/css").await.unwrap();

    std::process::Command::new("node_modules/.bin/tailwindcss")
        .args(["-i", "./src/main.css", "-o", "./dist/css/main.css"])
        .output()
        .expect("failed to build css");

    let css_file = tokio::fs::read("./dist/css/main.css").await.unwrap();
    let css_file_str = String::from_utf8_lossy(&css_file);

    let mut stylesheet =
        StyleSheet::parse(css_file_str.as_ref(), ParserOptions::default()).unwrap();

    stylesheet.minify(MinifyOptions::default()).unwrap();

    let css_file_min = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .unwrap();

    tokio::fs::write("./dist/css/main.min.css", css_file_min.code)
        .await
        .unwrap();
}
