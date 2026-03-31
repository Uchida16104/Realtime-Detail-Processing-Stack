use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub mode: String,
    pub data: String,
}

#[derive(Deserialize)]
pub struct NetworkCheckRequest {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct SecurityCheckRequest {
    pub input: String,
}

#[derive(Deserialize)]
pub struct FileCheckRequest {
    pub name: String,
    pub size: usize,
    pub content: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
    pub detail: Value,
}

#[derive(Serialize)]
pub struct StreamEvent {
    pub kind: String,
    pub status: String,
    pub timestamp: String,
    pub detail: Value,
}
