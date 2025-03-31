#![doc = include_str!("../README.md")]

mod api_calls;
mod request;
pub mod structures;
pub use api_calls::fingerprint_calls::cf_fingerprint;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    ReqwestError(#[from] reqwest::Error),
    URLParseError(#[from] url::ParseError),
}
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// A container to store the API key and invoke API calls on
///
/// ## Initialisation
/// ```rust
/// # use furse::Furse;
/// # tokio_test::block_on(async {
/// let curseforge = Furse::new(env!("CURSEFORGE_API_KEY"));
/// // Use the instance to call the API
/// let terralith = curseforge.get_mod(513688).await?;
/// # Ok::<_, furse::Error>(()) }).unwrap()
/// ```
#[derive(Clone, Debug)]
pub struct Furse {
    client: reqwest::Client,
    api_key: String,
}

impl Furse {
    /// Create a new API instance
    /// 
    /// Generate an API key in the [console](https://console.curseforge.com/#/api-keys).
    ///
    /// ```rust
    /// # use furse::Furse;
    /// let curseforge = Furse::new(env!("CURSEFORGE_API_KEY"));
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }
}
