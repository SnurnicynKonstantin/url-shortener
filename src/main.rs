use crate::storage::Storage;
use crate::handler::process_command;
use crate::controller::{shorten_url, redirect_to_url};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod storage;
mod command;
mod response;
mod error;
mod handler;
mod controller;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = Arc::new(Storage::new());

    let app = Router::new()
        .route("/getShortUrl", post(shorten_url))
        .route("/:short_url", get(redirect_to_url))
        .layer(CorsLayer::permissive())
        .with_state(storage);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}