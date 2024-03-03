use reqwest;

use super::types::Income;
use crate::config::Config;

impl Income {
    pub async fn get() -> Vec<Self> {
        let cfg = Config::get().await;
        let url = format!("https://api.emcd.io/v1/btc/income/{}", cfg.emcd_key);

        let mut income_data: serde_json::Value = reqwest::get(url)
            .await
            .unwrap().json().await
            .expect("Error in parsing stats");

        serde_json::from_value::<Vec<Self>>(income_data["income"].take())
            .expect("Error in parse income data")
    }
}