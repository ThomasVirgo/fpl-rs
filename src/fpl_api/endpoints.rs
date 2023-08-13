pub enum FPLEndpoint {
    Overview,
    Fixtures,
    FixturesByGameweek { event_id: i32 },
    PlayerSummary { element_id: i32 },
    GameweekInfo { event_id: i32 },
    ManagerSummary { manager_id: i32 },
    ManagerHistory { manager_id: i32 },
    ManagerTransfers { manager_id: i32 },
    ManagerTeam { manager_id: i32, event_id: i32 },
    ClassicLeagueStandings { league_id: i32, page_standings: i32 },
    H2HStandings { league_id: i32 },
}

const BASE_URL: &str = "https://fantasy.premierleague.com/api";

pub fn get_fpl_url(endpoint: &FPLEndpoint) -> String {
    match endpoint {
        FPLEndpoint::Overview => format!("{}/bootstrap-static/", BASE_URL),
        FPLEndpoint::Fixtures => format!("{}/fixtures/", BASE_URL),
        FPLEndpoint::FixturesByGameweek { event_id } => {
            format!("{}/fixtures/?event={}", BASE_URL, event_id)
        }
        FPLEndpoint::PlayerSummary { element_id } => {
            format!("{}/element-summary/{}/", BASE_URL, element_id)
        }
        FPLEndpoint::GameweekInfo { event_id } => {
            format!("{}/event/{}/live/", BASE_URL, event_id)
        }
        FPLEndpoint::ManagerSummary { manager_id } => format!("{}/entry/{}/", BASE_URL, manager_id),
        FPLEndpoint::ManagerHistory { manager_id } => {
            format!("{}/entry/{}/history/", BASE_URL, manager_id)
        }
        FPLEndpoint::ManagerTeam {
            manager_id,
            event_id,
        } => format!(
            "{}/entry/{}/event/{}/picks/",
            BASE_URL, manager_id, event_id
        ),
        FPLEndpoint::ManagerTransfers { manager_id } => {
            format!("{}/entry/{}/transfers/", BASE_URL, manager_id)
        }
        FPLEndpoint::ClassicLeagueStandings {
            league_id,
            page_standings,
        } => {
            format!(
                "{}/leagues-classic/{}/standings/?page_new_entries=1&page_standings={}&phase=1",
                BASE_URL, league_id, page_standings
            )
        }
        FPLEndpoint::H2HStandings { league_id } => {
            format!("{}/leagues-h2h-matches/league/{}/", BASE_URL, league_id)
        }
    }
}
