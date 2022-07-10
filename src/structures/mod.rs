pub mod common_structs;
pub mod file_structs;
pub mod fingerprint_structs;
pub mod mod_structs;

pub type UtcTime = chrono::DateTime<chrono::Utc>;
pub type Number = usize;
pub type ID = i32;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::Url;
