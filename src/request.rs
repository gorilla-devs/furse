use crate::{api_calls::Response, Furse, Result};
use reqwest::{IntoUrl, Url};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::LazyLock;

pub(crate) static API_URL_BASE: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://api.curseforge.com/v1/").unwrap());

impl Furse {
    /// Perform a GET request to `url` and deserialise to `T`
    pub(crate) async fn get<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<Response<T>> {
        let bytes = self
            .client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Perform a POST request to `url` with `body`
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        url: impl IntoUrl,
        body: &B,
    ) -> Result<Response<T>> {
        let bytes = self
            .client
            .post(url)
            .json(body)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
