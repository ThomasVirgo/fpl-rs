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
    id: u32,
    first_name: String,
    second_name: String,
    now_cost: u32,
    points_per_game: String,
    selected_by_percent: String,
    element_type: u32,
    photo: String,
    team: u32,
    total_points: u32,
    minutes: u32,
    starts: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Team {
    id: u32,
    name: String,
    points: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerType {
    id: u32,
    singular_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Overview {
    events: Vec<Event>,
    teams: Vec<Team>,
    elements: Vec<Player>,
    element_types: Vec<PlayerType>,
}
