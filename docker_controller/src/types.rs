use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SessionRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub container_id: String,
    pub editor_url: String,
}

#[derive(Debug, Deserialize)]
pub struct StopRequest {
    pub container_id: String,
}
