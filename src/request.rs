use crate::{Furse, Result};
use reqwest::{IntoUrl, Response};

const API_URL_BASE: &str = "https://api.curseforge.com/v1";

/// Perform a GET request to the API base url + `route` using `client`
pub(crate) async fn request_rel(client: &Furse, route: String) -> Result<Response> {
    request(client, format!("{}{}", API_URL_BASE, route)).await
}

/// Perform a GET request to `url` using `client`
pub(crate) async fn request(client: &Furse, url: impl IntoUrl) -> Result<Response> {
    let request = client.client.get(url).header("x-api-key", &client.api_key);

    request.send().await?.error_for_status()
}
