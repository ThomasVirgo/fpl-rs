use crate::fpl_api::fpl_schemas::response_trait::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ManagerTransfers(Vec<Transfer>);

#[derive(Deserialize, Serialize, Debug)]
pub struct Transfer {
    element_in: i32,
    element_in_cost: i32,
    element_out: i32,
    element_out_cost: i32,
    entry: i32,
    event: i32,
    time: String,
}

impl ApiResponse for ManagerTransfers {
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
