use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub msg: String,
}

impl <T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            msg: "success".to_string(),
        }
    }
    pub fn bad_request(msg: &str) -> Self {
        Self {
            code: 400,
            data: None,
            msg: msg.to_string(),
        }
    }
    pub fn not_found(msg: &str) -> Self {
        Self {
            code: 404,
            data: None,
            msg: msg.to_string(),
        }
    }
    pub fn authentication(msg: &str) -> Self {
        Self {
            code: 401,
            data: None,
            msg: msg.to_string(),
        }
    }
    
    pub fn error(code: i32, msg: &str) -> Self {
        Self {
            code,
            data: None,
            msg: msg.to_string(),
        }
    }
}
