use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: String,
}
