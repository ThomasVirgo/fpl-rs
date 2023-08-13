use crate::fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use crate::fpl_api::types::{LeagueStandings, ManagerSummary, Overview};
use reqwest::Error;

pub async fn pull_overview() -> Result<Overview, Error> {
    let request_url = get_fpl_url(&FPLEndpoint::Overview);
    let response = reqwest::get(&request_url).await?;
    let data: Overview = response.json().await?;
    Ok(data)
}

pub async fn pull_league_standings(league_id: i32, page: i32) -> Result<LeagueStandings, Error> {
    let request_url = get_fpl_url(&FPLEndpoint::ClassicLeagueStandings {
        league_id,
        page_standings: page,
    });
    let response = reqwest::get(&request_url).await?;
    let data: LeagueStandings = response.json().await?;
    Ok(data)
}

pub async fn pull_manager(manager_id: i32) -> Result<ManagerSummary, Error> {
    let request_url = get_fpl_url(&&FPLEndpoint::ManagerSummary {
        manager_id: manager_id,
    });
    let response = reqwest::get(&request_url).await?;
    let data: ManagerSummary = response.json().await?;
    Ok(data)
}
