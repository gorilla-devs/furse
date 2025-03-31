pub mod file_calls;
pub mod fingerprint_calls;
pub mod mod_calls;

pub(crate) use crate::{request::API_URL_BASE, structures::*, Furse, Result};

/// API responses are returned in this structure, with the actual results in [`data`] and optional [`pagination`]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<common_structs::Pagination>,
}
