use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Hashrate {
    pub hashrate: f32,
    pub hashrate_unit: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Mining {
    pub crypto: String,
    pub pool: String,
    pub hashrate: Hashrate,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Info {
    pub system: String,
    pub status: String,
    pub uptime: String,
    pub name: String,
    pub hot: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Hardware {
    pub name: String,
    pub temp: i32,
    pub fan: i32,
    pub speed: f32, 
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Worker {
    pub info: Info,
    pub mining: Mining,
    pub hardware: Vec<Hardware>
}