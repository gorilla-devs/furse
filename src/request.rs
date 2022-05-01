use crate::{Furse, Result};
use reqwest::{IntoUrl, Response, Url};

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.curseforge.com/v1/").unwrap();
}

/// Perform a GET request on `url` using the HTTPS client and user agent from `client`
pub(crate) async fn request(client: &Furse, url: impl IntoUrl) -> Result<Response> {
    Ok(client
        .client
        .get(url)
        .header("x-api-key", &client.api_key)
        .send()
        .await?
        .error_for_status()?)
}
