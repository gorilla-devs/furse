mod file_calls;
pub(crate) mod fingerprint_calls;
mod mod_calls;

use crate::{request::API_URL_BASE, structures::*, Furse, Result};

/// API responses are returned in this structure, with the actual results in `data` and optional `pagination`
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<common_structs::Pagination>,
}
