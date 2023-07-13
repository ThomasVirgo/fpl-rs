use crate::fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use crate::fpl_api::types::Overview;
use reqwest::Error;

pub async fn pull_overview() -> Result<Overview, Error> {
    let request_url = get_fpl_url(FPLEndpoint::Overview);
    let response = reqwest::get(&request_url).await?;
    let data: Overview = response.json().await?;
    Ok(data)
}
