use crate::fpl_api::types::Overview;
use crate::ManagerTeam;
use crate::PlayerStatsForGameweek;
use std::collections::HashMap;

pub fn ids_difference(first_ids: Vec<i32>, second_ids: Vec<i32>) -> Vec<i32> {
    first_ids
        .into_iter()
        .filter(|&id| !second_ids.contains(&id))
        .collect()
}

pub fn element_to_name_mapping(overview: Overview) -> HashMap<i32, String> {
    let mut element_to_name = HashMap::new();
    for player in overview.elements {
        let full_name = format!("{} {}", player.first_name, player.second_name);
        element_to_name.insert(player.player_id, full_name);
    }
    element_to_name
}

pub struct GameweekInfo {
    pub gameweek: i32,
    pub player_stats: PlayerStatsForGameweek,
    pub manager_team: ManagerTeam,
}

impl GameweekInfo {
    fn get_element_id_of_captain(&self) -> Option<i32> {
        for pick in self.manager_team.picks.iter() {
            if pick.is_captain {
                return Some(pick.element);
            }
        }
        return None;
    }

    fn element_id_to_total_score(&self) -> HashMap<i32, i32> {
        let mut id_to_score: HashMap<i32, i32> = HashMap::new();
        for player_points in self.player_stats.elements.iter() {
            id_to_score.insert(player_points.id, player_points.stats.total_points);
        }
        id_to_score
    }

    fn get_max_score(&self) -> i32 {
        let mut max_score = 0;
        let id_to_score = self.element_id_to_total_score();
        for pick in self.manager_team.picks.iter() {
            let score = id_to_score.get(&pick.element).unwrap();
            if score > &max_score {
                max_score = *score;
            }
        }
        max_score
    }

    /// returns the element ids that had the maximum score in the managers team
    fn get_max_scoring_elements(&self) -> Vec<i32> {
        let mut max_scorers = Vec::new();
        let max_score = self.get_max_score();
        let id_to_score = self.element_id_to_total_score();
        for pick in self.manager_team.picks.iter() {
            let score = id_to_score.get(&pick.element).unwrap();
            if score == &max_score {
                max_scorers.push(pick.element);
            }
        }
        max_scorers
    }

    fn was_captain_correct(&self) -> bool {
        let captain_id = self.get_element_id_of_captain();
        match captain_id {
            Some(id) => self.get_max_scoring_elements().contains(&id),
            None => false,
        }
    }
}

struct CaptaincyScoring {
    correct_captaincy_gameweeks: Vec<i32>,
    incorrect_captaincy_gameweeks: Vec<i32>,
    captains_chosen: Vec<String>,
    total_points_lost_to_incorrect_captaincy: i32,
}

pub fn extract_correct_captaincy(overview: Overview, gameweek_infos: Vec<GameweekInfo>) {
    let element_to_name = element_to_name_mapping(overview);
}
