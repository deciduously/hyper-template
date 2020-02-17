// handlers.rs
// Web route handlers and router

use crate::templates::*;
use askama::Template;
use flate2::{write::ZlibEncoder, Compression};
use hyper::{header, Body, Response, StatusCode};
use std::{convert::Infallible, fs::File, io::prelude::*, path::PathBuf};

// Universal handler return type
pub type HandlerResult = Result<Response<Body>, Infallible>;

// General handlers

/// Top-level handler that DEFLATE compresses and responds with from a &[u8] body
/// If None passed to status, 200 OK will be returned
pub async fn bytes_handler(
    body: &[u8],
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    // Compress
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(body).unwrap();
    let compressed = e.finish().unwrap();
    // Return response
    Ok(Response::builder()
        .status(status.unwrap_or_default())
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_ENCODING, "deflate")
        .body(Body::from(compressed))
        .unwrap())
}

/// Pass string through to bytes_handler
pub async fn string_handler(
    body: &str,
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    bytes_handler(body.as_bytes(), content_type, status).await
}

pub async fn html_str_handler(body: &str) -> HandlerResult {
    string_handler(body, "text/html", None).await
}

// Route handlers

pub async fn image(path_str: &str) -> HandlerResult {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    if let Some(ext) = path_buf.extension() {
        match ext.to_str().unwrap() {
            "ico" => {
                let mut file =
                    File::open("src/assets/images/favicon.ico").expect("Should open icon file");
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).expect("Should read icon file");
                bytes_handler(&buf, "image/x-icon", None).await
            }
            "svg" => {
                // build the response
                let xml = match file_name {
                    // "dev-badge.svg" => include_str!("assets/images/dev-badge.svg"), // for example
                    _ => "",
                };
                string_handler(xml, "image/svg+xml", None).await
            }
            _ => four_oh_four().await,
        }
    } else {
        four_oh_four().await
    }
}

pub async fn index() -> HandlerResult {
    let template = IndexTemplate::default();
    let html = template.render().expect("Should render markup");
    html_str_handler(&html).await
}

pub async fn four_oh_four() -> HandlerResult {
    let template = FourOhFourTemplate::default();
    let html = template.render().expect("Should render markup");
    string_handler(&html, "text/html", Some(StatusCode::NOT_FOUND)).await
}
