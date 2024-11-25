

#[derive(Clone)]
pub struct AuthState {
    pub secret_key: String,
}

impl AuthState {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}