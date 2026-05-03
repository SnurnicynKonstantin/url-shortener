use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    response::Redirect,
    Json as RequestJson,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::storage::Storage;
use crate::handler::process_command;
use crate::response::Response;

const URL_TTL_SECONDS: u64 = 300;

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

pub async fn shorten_url(
    State(storage): State<Arc<Storage>>,
    RequestJson(payload): RequestJson<ShortenRequest>,
) -> Result<Json<ShortenResponse>, StatusCode> {
    let command = format!("SETURL {}", payload.url);
    
    match process_command(&storage, &command) {
        Ok(Response::BulkString(Some(short_key))) => {
            let set_ttl_command = format!("SETTTL {} {}", short_key, URL_TTL_SECONDS);
            if let Err(_) = process_command(&storage, &set_ttl_command) {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            
            let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
            let short_url = format!("http://localhost:{}/{}", port, short_key);
            Ok(Json(ShortenResponse { short_url }))
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn redirect_to_url(
    Path(short_url): Path<String>,
    State(storage): State<Arc<Storage>>,
) -> Result<Redirect, StatusCode> {
    match storage.get(&short_url) {
        Ok(Response::BulkString(Some(url))) => {
            let set_ttl_command = format!("SETTTL {} {}", short_url, URL_TTL_SECONDS);
            let _ = process_command(&storage, &set_ttl_command);
            
            Ok(Redirect::permanent(&url))
        }
        Ok(Response::BulkString(None)) | Ok(Response::Null) => {
            Err(StatusCode::NOT_FOUND)
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}