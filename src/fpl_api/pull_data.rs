use crate::fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use crate::fpl_api::types::{LeagueStandings, Overview};
use reqwest::Error;

pub async fn pull_overview() -> Result<Overview, Error> {
    let request_url = get_fpl_url(FPLEndpoint::Overview);
    let response = reqwest::get(&request_url).await?;
    let data: Overview = response.json().await?;
    Ok(data)
}

pub async fn pull_league_standings(league_id: u32, page: i32) -> Result<LeagueStandings, Error> {
    let request_url = get_fpl_url(FPLEndpoint::ClassicLeagueStandings {
        league_id,
        page_standings: page,
    });
    let response = reqwest::get(&request_url).await?;
    let data: LeagueStandings = response.json().await?;
    Ok(data)
}
