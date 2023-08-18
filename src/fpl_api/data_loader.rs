use crate::fpl_api::endpoints::{get_fpl_url, FPLEndpoint};
use crate::fpl_api::fpl_schemas::response_trait::ApiResponse;
use std::error;

pub async fn get_data_for_endpoint<T: ApiResponse>(
    endpoint: FPLEndpoint,
) -> Result<T, Box<dyn error::Error>> {
    let url = get_fpl_url(&endpoint);
    let data = fetch_and_deserialize::<T>(&url).await?;
    Ok(data)
}

async fn fetch_and_deserialize<T: ApiResponse>(url: &str) -> Result<T, Box<dyn error::Error>> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let deserialized: T = T::from_json(&text)?;
    Ok(deserialized)
}
