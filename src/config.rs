
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub emcd_key: String,
    pub tg_token: String,
    pub ms_access_key: String,
    pub asics: Vec<String>
}

impl Config {
    pub async fn get() -> Self {
        envy::from_env::<Self>()
            .expect("Error at parsing env")
    }
}