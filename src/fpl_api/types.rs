use serde::{Deserialize, Serialize};

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
