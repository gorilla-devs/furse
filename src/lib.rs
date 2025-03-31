#![doc = include_str!("../README.md")]

mod api_calls;
mod request;
pub mod structures;
pub use api_calls::fingerprint_calls::cf_fingerprint;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", .0)]
    URLParseError(#[from] url::ParseError),
}
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An instance of the API to invoke API calls on
///
/// To initialise this container,
/// ```rust
/// # use furse::Furse;
/// # tokio_test::block_on(async {
/// let curseforge = Furse::new(env!("CURSEFORGE_API_KEY"));
/// // Use the instance to call the API
/// let terralith_mod = curseforge.get_mod(513688).await?;
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
    /// `api_key` should be a CurseForge API key.
    /// You can obtain one by applying [here](https://forms.monday.com/forms/dce5ccb7afda9a1c21dab1a1aa1d84eb?r=use1)
    ///
    /// ```rust
    /// # use furse::Furse;
    /// let curseforge = Furse::new(env!("CURSEFORGE_API_KEY"));
    /// ```
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }
}
