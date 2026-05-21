use url_shortener::storage::Storage;
use url_shortener::controller::{shorten_url, redirect_to_url};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'a', long, default_value = "127.0.0.1")]
    host: String,
    
    #[clap(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let storage = Arc::new(Storage::new());

    let app = Router::new()
        .route("/getShortUrl", post(shorten_url))
        .route("/:short_url", get(redirect_to_url))
        .layer(CorsLayer::permissive())
        .with_state(storage);

    let addr = format!("{}:{}", args.host, args.port);
    println!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}