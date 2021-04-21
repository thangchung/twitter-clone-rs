extern crate serde;

pub mod repositories;
pub mod services;

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret_key: String,
    pub jwt_lifetime: i64,
}
