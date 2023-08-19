use crate::fpl_api::fpl_schemas::response_trait::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerStatsForGameweek {
    elements: Vec<PlayerPoints>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerPoints {
    id: i32,
    stats: Stats,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stats {
    minutes: i32,
    goals_scored: i32,
    assists: i32,
    clean_sheets: i32,
    goals_conceded: i32,
    own_goals: i32,
    penalties_saved: i32,
    penalties_missed: i32,
    yellow_cards: i32,
    red_cards: i32,
    saves: i32,
    bonus: i32,
    bps: i32,
    influence: String,
    creativity: String,
    threat: String,
    ict_index: String,
    starts: i32,
    expected_goals: String,
    expected_assists: String,
    expected_goal_involvements: String,
    expected_goals_conceded: String,
    total_points: i32,
    in_dreamteam: bool,
}

impl ApiResponse for PlayerStatsForGameweek {
    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
