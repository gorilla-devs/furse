pub mod file_calls;
pub mod mod_calls;
use super::structures::common_structs::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<Pagination>,
}
