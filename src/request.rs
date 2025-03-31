use crate::{api_calls::Response, Furse, Result};
use reqwest::{IntoUrl, Url};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::LazyLock;

pub(crate) static API_URL_BASE: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://api.curseforge.com/v1/").unwrap());

impl Furse {
    /// Perform a GET request to `url` and deserialise to `T`
    pub(crate) async fn get<T>(&self, url: impl IntoUrl) -> Result<Response<T>>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Perform a GET request to `url` with `body`
    pub(crate) async fn post<T, B>(&self, url: impl IntoUrl, body: &B) -> Result<Response<T>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        Ok(self
            .client
            .post(url)
            .json(body)
            .header("x-api-key", &self.api_key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
