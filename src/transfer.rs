use std::cell::RefCell;

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
    pub Message: String,
    pub Total:   usize,
    pub Invalid: u32,
    pub Latency: u64
}

#[derive(Serialize, Deserialize)]
pub struct SimpleRpcResponse {
    pub Code: u32
}

pub struct Database {
    pub methods: RefCell<Vec<FalconMethod>>
}
