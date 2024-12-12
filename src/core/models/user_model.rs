use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}