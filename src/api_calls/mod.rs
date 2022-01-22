pub mod file_calls;
pub mod mod_calls;
use super::structures::common_structs::Pagination;
use serde::{Deserialize, Serialize};

// For some reason CurseForge returns API responses in a 'response' json structure with the _actual_ results in `body`
// Sometimes there are paginations so uhh yeah that too
#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<Pagination>,
}
