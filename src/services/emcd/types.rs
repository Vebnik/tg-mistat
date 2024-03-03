use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Income {
    pub timestamp: f64,
    pub gmt_time: String,
    pub income: f64,
    pub total_hashrate: i64,
}
