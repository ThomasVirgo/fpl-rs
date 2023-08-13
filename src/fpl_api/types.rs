use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    id: u32,
    deadline_time_epoch: u32,
    most_captained: Option<u32>,
    most_selected: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
    #[serde(rename = "id")]
    pub player_id: i32,
    pub first_name: String,
    pub second_name: String,
    pub now_cost: i32,
    pub points_per_game: String,
    pub selected_by_percent: String,
    pub element_type: i32,
    pub photo: String,
    pub team: i32,
    pub total_points: i32,
    pub minutes: i32,
    pub starts: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PlayerFromDB {
    pub player_id: i32,
    pub first_name: String,
    pub second_name: String,
    pub now_cost: i32,
    pub points_per_game: f32,
    pub selected_by_percent: f32,
    pub element_type: i32,
    pub photo: String,
    pub team: i32,
    pub total_points: i32,
    pub minutes: i32,
    pub starts: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Team {
    id: i32,
    name: String,
    points: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerType {
    id: i32,
    singular_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Overview {
    pub events: Vec<Event>,
    pub teams: Vec<Team>,
    pub elements: Vec<Player>,
    pub element_types: Vec<PlayerType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManagerFplWrapped {
    manager_id: i32,
    best_gw_score: i32,
    best_gw_rank: i32,
    best_overall_rank: i32,
    most_played_player_id: i32,
    correct_captaincy_picks: i32,
    incorrect_captaincy_picks: i32,
    points_lost_from_incorrect_captaincy: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Standings {
    pub results: Vec<Manager>,
    pub has_next: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueStandings {
    pub standings: Standings,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Manager {
    pub player_name: String,
    pub entry: i32,
    pub entry_name: String,
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct ManagerDB {
    pub player_name: String,
    pub manager_id: i32,
    pub entry_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManagerSummary {
    pub id: i32,
    pub player_first_name: String,
    pub player_last_name: String,
    pub name: String,
}
