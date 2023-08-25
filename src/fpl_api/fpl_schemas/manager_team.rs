use crate::fpl_api::fpl_schemas::response_trait::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ManagerTeam {
    active_chip: Option<String>,
    //automatic subs
    entry_history: EntryHistory,
    pub picks: Vec<Pick>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EntryHistory {
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
pub struct Pick {
    pub element: i32,
    position: i32,
    multiplier: i32,
    pub is_captain: bool,
    is_vice_captain: bool,
}

impl ApiResponse for ManagerTeam {
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
