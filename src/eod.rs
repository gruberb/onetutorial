use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EODResponse {
    pub code: String,
    pub close: f64,
}

