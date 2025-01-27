use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use lru::LruCache;
use serde::Serialize;
use std::{collections::BTreeMap, num::NonZero, sync::Arc};
use chrono::prelude::*;

use tokio::sync::RwLock as TokioRWLock;

struct Entry {
    value: String,
    time: DateTime<Utc>,
}

type SharedMap = Arc<TokioRWLock<LruCache<String, Entry>>>;

#[tokio::main]
async fn main() {

    // Read the address and port from the command line arguments
    let args: Vec<String> = std::env::args().collect();
    let address = if args.len() > 1 {
        &args[1]
    } else {
        // Some reasonable default for local development
        "127.0.0.1:3000"
    };

    // Aim for 256 MiB max size and 128 bytes per entry
    const MAX_ELEMENTS: usize = ( 256 * 1024 * 1024  ) / 128;

    // Initialize the shared map
    let max_elements = NonZero::new(MAX_ELEMENTS).unwrap();
    let shared_map: SharedMap = Arc::new(TokioRWLock::new(LruCache::new(max_elements)));

    // Build the Axum router
    let app = Router::new()
        .route("/entries/{*id}", get(read_handler).post(write_handler).delete(delete_handler))
        .route("/all_entries/{*id_prefix}", get(read_all_handler).delete(delete_all_handler))
        .route("/infos/{*id}", get(info_handler))
        .route("/all_infos/{*id_prefix}", get(all_infos_handler))
        .route("/health", get(health_check))
        .route("/status", get(status))
        .with_state(shared_map);

    // Create a TCP listener
    // If the address is invalid, just explode, there is no point in handling this error
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();

    println!("Vaping on [ {} ]", address);

    // Run the Axum server
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn write_handler(
    Path(key): Path<String>,
    State(db): State<SharedMap>,
    payload: String
) -> impl IntoResponse {
    let mut map = db.write().await;

    let entry = Entry {
        value: payload,
        time: Utc::now(),
    };

    map.put(key, entry);
    StatusCode::OK
}

async fn read_handler(
    Path(key): Path<String>,
    State(db): State<SharedMap>,
) -> impl IntoResponse {
    let map = db.read().await;
    if let Some(entry) = map.peek(&key) {
        (StatusCode::OK, entry.value.clone()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn read_all_handler(
    Path(id_prefix): Path<String>,
    State(db): State<SharedMap>,
) -> impl IntoResponse {
    let map = db.read().await;
    let entries: BTreeMap<String, String> = map.iter()
        .filter(|(key, _)| key.starts_with(&id_prefix))
        .map(|(key, entry)| (key.clone(), entry.value.clone()))
        .collect();
    Json(entries).into_response()
}

async fn delete_handler(
    Path(key): Path<String>,
    State(db): State<SharedMap>,
) -> impl IntoResponse {
    let mut map = db.write().await;
    if map.pop(&key).is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn delete_all_handler(
    Path(id_prefix): Path<String>,
    State(db): State<SharedMap>,
) -> impl IntoResponse {
    let mut map = db.write().await;
    let keys_to_delete: Vec<String> = map.iter()
        .filter(|(key, _)| key.starts_with(&id_prefix))
        .map(|(key, _)| key.clone())
        .collect();
    for key in keys_to_delete {
        map.pop(&key);
    }
    StatusCode::OK
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}

#[derive(Serialize)]
pub struct DBStatus {
    pub num_entries: usize,
    pub db_version: String,
}

async fn status(State(db): State<SharedMap>) -> impl IntoResponse {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let map = db.read().await;
    let num_entries = map.len();
    Json( DBStatus { num_entries, db_version: VERSION.into() } ).into_response()
}

#[derive(Serialize)]
struct InfoResponse {
    pub value: String,
    // When serialized to json, this will be a human-readable string
    pub time: DateTime<Utc>,
}

async fn info_handler(
    Path(key): Path<String>,
    State(db): State<SharedMap>,
) -> impl IntoResponse {
    let  map = db.read().await;
    if let Some(entry) = map.peek(&key) {
        Json(InfoResponse {
            value: entry.value.clone(),
            time: entry.time,
        }).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn all_infos_handler(
    Path(id_prefix): Path<String>,
    State(db): State<SharedMap>,
) -> impl IntoResponse {
    let map = db.read().await;
    let entries: BTreeMap<String, InfoResponse> = map.iter()
        .filter(|(key, _)| key.starts_with(&id_prefix))
        .map(|(key, entry)| (key.clone(), InfoResponse {
            value: entry.value.clone(),
            time: entry.time,
        }))
        .collect();
    Json(entries).into_response()
}
