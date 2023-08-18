use crate::fpl_api::fpl_schemas::response_trait::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ManagerHistory {
    current: Vec<CurrentEvent>,
    past: Vec<PastEvent>,
    // chips (not sure on form yet)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CurrentEvent {
    event: i32,
    points: i32,
    total_points: i32,
    rank: i32,
    rank_sort: i32,
    overall_rank: i32,
    bank: i32,
    value: i32,
    event_transfers: i32,
    event_transfers_cost: i32,
    points_on_bench: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PastEvent {
    season_name: String,
    total_points: i32,
    rank: i32,
}

impl ApiResponse for ManagerHistory {
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
