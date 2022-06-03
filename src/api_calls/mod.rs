pub mod file_calls;
pub mod mod_calls;
pub mod fingerprint_calls;
use super::structures::common_structs::Pagination;
use serde::{Deserialize, Serialize};

// CurseForge returns API responses in a 'response' json structure with the actual results in `body` and `pagination`s too
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<Pagination>,
}
