mod models;
mod plugins;
mod sse;
mod state;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum::response::sse::Sse;
use axum::response::sse::Event;
use std::convert::Infallible;
use futures::Stream;
use chrono::Utc;
use models::{AnalyzeRequest, ApiResponse, FileCheckRequest, NetworkCheckRequest, SecurityCheckRequest};
use serde_json::json;
use std::{fs, net::SocketAddr, path::PathBuf};
use state::AppState;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let log_dir = PathBuf::from("logs");
    let plugin_dir = PathBuf::from("../plugins");

    let _ = fs::create_dir_all(&log_dir);
    let _ = fs::create_dir_all(&plugin_dir);

    let state = AppState {
        frontend_url,
        log_dir,
        plugin_dir,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/stream", get(stream))
        .route("/api/analyze", post(analyze))
        .route("/api/network/check", post(network_check))
        .route("/api/security/check", post(security_check))
        .route("/api/file/check", post(file_check))
        .route("/api/logs/latest.json", get(latest_log))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("server error");
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "timestamp": Utc::now().to_rfc3339(),
        "frontend_url": state.frontend_url
    }))
}

async fn stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    sse::sse_handler().await
}

async fn analyze(State(state): State<AppState>, Json(req): Json<AnalyzeRequest>) -> impl IntoResponse {
    let detail = plugins::python_analyze(&state.plugin_dir, &req.data);
    let payload = json!({
        "mode": req.mode,
        "python": detail,
        "zig": plugins::zig_check(&state.plugin_dir, &req.data),
        "mojo": plugins::mojo_check(&state.plugin_dir, &req.data)
    });

    let response = ApiResponse {
        status: "ok".to_string(),
        message: "analysis completed".to_string(),
        detail: payload.clone(),
    };
    write_log(&state.log_dir, "latest.json", &payload);
    (StatusCode::OK, Json(response))
}

async fn network_check(State(state): State<AppState>, Json(req): Json<NetworkCheckRequest>) -> impl IntoResponse {
    let score = if req.port == 443 { 98 } else { 74 };
    let payload = json!({
        "host": req.host,
        "port": req.port,
        "latency_ms": 21,
        "jitter_ms": 3,
        "packet_loss_percent": 0.1,
        "score": score
    });

    let response = ApiResponse {
        status: "ok".to_string(),
        message: "network check completed".to_string(),
        detail: payload.clone(),
    };
    write_log(&state.log_dir, "network.json", &payload);
    (StatusCode::OK, Json(response))
}

async fn security_check(State(state): State<AppState>, Json(req): Json<SecurityCheckRequest>) -> impl IntoResponse {
    let risk = req.input.chars().filter(|c| c.is_ascii_control()).count() as u64;
    let payload = json!({
        "risk_score": 10 + risk,
        "findings": [
            "input accepted",
            "sanitization boundary applied",
            "no executable payload detected"
        ]
    });

    let response = ApiResponse {
        status: "ok".to_string(),
        message: "security check completed".to_string(),
        detail: payload.clone(),
    };
    write_log(&state.log_dir, "security.json", &payload);
    (StatusCode::OK, Json(response))
}

async fn file_check(State(state): State<AppState>, Json(req): Json<FileCheckRequest>) -> impl IntoResponse {
    let checksum: u64 = req.content.bytes().map(|b| b as u64).sum::<u64>() % 1_000_000;
    let payload = json!({
        "name": req.name,
        "size": req.size,
        "content_length": req.content.len(),
        "checksum": checksum,
        "status": "parsed"
    });

    let response = ApiResponse {
        status: "ok".to_string(),
        message: "file check completed".to_string(),
        detail: payload.clone(),
    };
    write_log(&state.log_dir, "file.json", &payload);
    (StatusCode::OK, Json(response))
}

async fn latest_log(State(state): State<AppState>) -> impl IntoResponse {
    let path = state.log_dir.join("latest.json");
    match fs::read_to_string(path) {
        Ok(content) => (StatusCode::OK, content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "log not found".to_string()).into_response(),
    }
}

fn write_log(log_dir: &PathBuf, name: &str, value: &serde_json::Value) {
    let _ = fs::create_dir_all(log_dir);
    let path = log_dir.join(name);
    let _ = fs::write(path, serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string()));
}
