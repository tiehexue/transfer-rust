use std::cell::RefCell;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Metric {
    pub endpoint: String,
    pub metric: String,
    pub value: f64,
    pub step: u32,
    pub counterType: String,
    pub tags: String,
    pub timestamp: u64
}

#[derive(Serialize, Deserialize)]
pub struct FalconMethod {
    pub method: String,
    pub params: Vec<Vec<Metric>>,
    pub id: u32
}

#[derive(Serialize, Deserialize)]
pub struct TransferResponse {
    pub id: u64,
    pub result: Value,
    pub error: Value
}

pub struct Database {
    pub methods: RefCell<Vec<FalconMethod>>
}
