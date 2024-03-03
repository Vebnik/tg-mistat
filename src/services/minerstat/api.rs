use super::types::Worker;
use crate::config::Config;

use reqwest;

impl Worker {
    pub async fn get() -> Vec<Worker> {
        let cfg = Config::get().await;
        
        let url = format!("https://api.minerstat.com/v2/stats/{}/", cfg.ms_access_key);

        let res: serde_json::Value = reqwest::get(url)
            .await
            .unwrap().json().await
            .expect("Error in parsing stats");

        let data: Vec<Self> = res
            .as_object()
            .unwrap()
            .values()
            .map(|value| serde_json::from_value(value.clone()).unwrap()).collect();

        data
    }
}