use std::io::prelude::*;

use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use flate2::write::GzEncoder;
use flate2::Compression;
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};

pub async fn file_handler(headers: HeaderMap, Path(path): Path<String>) -> impl IntoResponse {
    let Some(accept_encoding) = headers.get(header::ACCEPT_ENCODING.as_str()) else {
        return (StatusCode::NOT_ACCEPTABLE, "NOT_ACCEPTABLE").into_response();
    };

    let file_path = format!("dist/{path}");

    if !tokio::fs::try_exists(&file_path).await.unwrap_or_default() {
        return (StatusCode::NOT_FOUND, "NOT_FOUND").into_response();
    }

    if accept_encoding.to_str().unwrap_or_default().contains("br") {
        let brotli_file_path = format!("{file_path}.br");

        if tokio::fs::try_exists(&brotli_file_path)
            .await
            .unwrap_or_default()
        {
            let Ok(content) = tokio::fs::read(&brotli_file_path).await else {
                return (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR").into_response();
            };

            let mime = mime_type_from_file(&file_path);

            return (
                StatusCode::OK,
                [
                    (header::CONTENT_ENCODING, "br"),
                    (header::CONTENT_TYPE, &mime),
                ],
                content,
            )
                .into_response();
        }
    }

    if accept_encoding.to_str().unwrap_or_default().contains("gz") {
        let gzip_file_path = format!("{file_path}.gz");

        if tokio::fs::try_exists(&gzip_file_path)
            .await
            .unwrap_or_default()
        {
            let Ok(content) = tokio::fs::read(&gzip_file_path).await else {
                return (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR").into_response();
            };

            let mime = mime_type_from_file(&file_path);

            return (
                StatusCode::OK,
                [
                    (header::CONTENT_ENCODING, "gz"),
                    (header::CONTENT_TYPE, &mime),
                ],
                content,
            )
                .into_response();
        }
    }

    let Ok(content) = tokio::fs::read(&file_path).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR").into_response();
    };

    let mime = mime_type_from_file(&file_path);

    (StatusCode::OK, [(header::CONTENT_TYPE, &mime)], content).into_response()
}

pub fn mime_type_from_file(file_path: &str) -> String {
    let content_type = match file_path.split('.').last() {
        Some(value) => match value {
            "js" => "application/javascript; charset=utf-8",
            "css" => "text/css",
            _ => "text/plain",
        },
        None => "text/plain",
    };

    content_type.to_string()
}

pub async fn compress_all() {
    tracing::info!("static compression");

    let mut tasks = tokio::task::JoinSet::new();

    for entry in walkdir::WalkDir::new("dist")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if entry_path.is_file() {
            if let Some(file_path) = entry_path.to_str() {
                let file_path = file_path.to_string();
                let file_path_cloned = file_path.clone();

                // brotli
                tasks.spawn(async move {
                    if let Ok(content) = tokio::fs::read(&file_path).await {
                        if let Ok(output) = std::fs::File::create(format!("{file_path}.br")) {
                            let mut output_compressed = brotlic::CompressorWriter::new(output);
                            output_compressed.write_all(&content).ok();
                        }
                    }
                });

                // gzip
                tasks.spawn(async move {
                    if let Ok(content) = tokio::fs::read(&file_path_cloned).await {
                        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                        if encoder.write_all(&content).is_ok() {
                            if let Ok(compressed) = encoder.finish() {
                                tokio::fs::write(format!("{file_path_cloned}.gz"), &compressed)
                                    .await
                                    .ok();
                            }
                        }
                    }
                });
            }
        }
    }

    while tasks.join_next().await.is_some() {
        continue;
    }
}

pub async fn build_static() {
    if tokio::fs::try_exists("./dist").await.unwrap() {
        tokio::fs::remove_dir_all("./dist").await.unwrap();
    }

    process_css().await;
    process_javascript().await;
}

async fn process_javascript() {
    tracing::info!("javascript build");

    tokio::fs::create_dir_all("./dist/js").await.unwrap();

    let htmx = reqwest::get("https://unpkg.com/htmx.org@1.9.4/dist/htmx.min.js")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    tokio::fs::write("./dist/js/htmx.min.js", &htmx)
        .await
        .unwrap();
}

async fn process_css() {
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

    tokio::fs::write("./dist/css/main.min.css", &css_file_min.code)
        .await
        .unwrap();
}
