pub enum FPLEndpoint {
    Overview,
    Fixtures,
    FixturesByGameweek { event_id: u32 },
    PlayerSummary { element_id: u32 },
    GameweekInfo { event_id: u32 },
    ManagerSummary { manager_id: u32 },
    ManagerHistory { manager_id: u32 },
    ManagerTransfers { manager_id: u32 },
    ManagerTeam { manager_id: u32, event_id: u32 },
    ClassicLeagueStandings { league_id: u32 },
    H2HStandings { league_id: u32 },
}

const BASE_URL: &str = "https://fantasy.premierleague.com/api";

pub fn get_fpl_url(endpoint: FPLEndpoint) -> String {
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
        FPLEndpoint::ClassicLeagueStandings { league_id } => {
            format!("{}/leagues-classic/{}/standings/", BASE_URL, league_id)
        }
        FPLEndpoint::H2HStandings { league_id } => {
            format!("{}/leagues-h2h-matches/league/{}/", BASE_URL, league_id)
        }
    }
}
