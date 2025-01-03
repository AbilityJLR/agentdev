use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub response: String,
}
