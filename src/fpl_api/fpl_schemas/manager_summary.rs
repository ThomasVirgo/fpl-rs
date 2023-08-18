use crate::fpl_api::fpl_schemas::response_trait::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ManagerSummary {
    id: i32,
    joined_time: String,
    started_event: i32,
    player_first_name: String,
    player_last_name: String,
    player_region_id: i32,
    player_region_name: String,
    player_region_iso_code_short: String,
    player_region_iso_code_long: String,
    summary_overall_points: i32,
    summary_overall_rank: i32,
    summary_event_points: i32,
    summary_event_rank: i32,
    current_event: i32,
    name: String,
    name_change_blocked: bool,
    last_deadline_bank: i32,
    last_deadline_value: i32,
    last_deadline_total_transfers: i32,
    leagues: Leagues,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Leagues {
    classic: Vec<League>,
    h2h: Vec<League>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct League {
    id: i32,
    name: String,
    created: String,
    closed: bool,
    start_event: i32,
    has_cup: bool,
    entry_rank: i32,
    entry_last_rank: i32,
}

impl ApiResponse for ManagerSummary {
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
