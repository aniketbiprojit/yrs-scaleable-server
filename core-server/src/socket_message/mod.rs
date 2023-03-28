pub mod handle_event;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocketMessage {
    pub message: String,
    pub event: String,
}
