use crate::handlers::*;
use hyper::{Body, Method, Request};
use log::{info, warn};

pub async fn router(req: Request<Body>) -> HandlerResult {
    let (method, path) = (req.method(), req.uri().path());
    info!("{} {}", method, path);
    match (method, path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        (&Method::GET, "/main.css") => {
            string_handler(include_str!("assets/main.css"), "text/css", None).await
        }
        (&Method::GET, "/manifest.json") => {
            string_handler(include_str!("assets/manifest.json"), "text/json", None).await
        }

        (&Method::GET, "/robots.txt") => {
            string_handler(include_str!("assets/robots.txt"), "text", None).await
        }
        (&Method::GET, path_str) => {
            // Otherwise...
            // is it an image?
            if let Some(ext) = path_str.split('.').nth(1) {
                match ext {
                    "ico" | "svg" => image(path).await,
                    _ => four_oh_four().await,
                }
            } else {
                four_oh_four().await
            }
        }
        _ => {
            warn!("{}: 404!", path);
            four_oh_four().await
        }
    }
}
