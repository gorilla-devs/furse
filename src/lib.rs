//! # Furse
//!
//! Furse is a simple library for using the [CurseForge API](https://docs.curseforge.com/) in Rust projects.
//! It uses [Reqwest](https://docs.rs/reqwest/) as its HTTPS client and deserialises responses to strongly typed structs using [SerDe](https://serde.rs/).
//!
//! ## Features
//!
//! This crate includes the following:
//! - API calls:
//!   - Get mod by mod ID (<https://docs.curseforge.com/#get-mod>)
//!   - Get mod's HTML description by ID (<https://docs.curseforge.com/#get-mod-description>)
//!   - Get mod's files by mod ID (<https://docs.curseforge.com/#get-mod-files>)
//!   - Get file by the mod ID and file ID (<https://docs.curseforge.com/#get-mod-file>)
//!   - Get file's HTML changelog by mod ID and file ID (<https://docs.curseforge.com/#get-mod-file-changelog>)
//!   - Download a file from a `File`
//!   - Download a file by mod ID and file ID (<https://docs.curseforge.com/#get-mod-file-download-url>)
//! - Schemas and their dependant schemas:
//!   - Mod <https://docs.curseforge.com/#tocS_Mod>
//!   - File <https://docs.curseforge.com/#tocS_File>
//!
//! This crate uses [Rustls](https://docs.rs/rustls/) rather than OpenSSL, because OpenSSL is outdated and slower.

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
/// # #[tokio::main]
/// # async fn main() -> Result<(), furse::Error> {
/// let curseforge = Furse::new(env!("CURSEFORGE_API_KEY"));
/// // Use the instance to call the API
/// let terralith_mod = curseforge.get_mod(513688).await?;
/// # Ok(()) }
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
