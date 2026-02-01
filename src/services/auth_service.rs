use crate::state::AppState;


#[derive(Debug)]
pub struct AuthService {
    state: AppState,
}

impl AuthService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn login(&self, email: &str, code: &str) -> Result<String, String> {
        Ok("mock_token_123456".to_string())
    }
}