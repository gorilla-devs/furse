pub mod common_structs;
pub mod file_structs;
pub mod fingerprint_structs;
pub mod mod_structs;

pub type UtcTime = chrono::DateTime<chrono::Utc>;
pub type Number = i64;
pub type ID = i32;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::Url;

fn deserialise_optional_url<'de, D: serde::Deserializer<'de>>(
    de: D,
) -> Result<Option<Url>, D::Error> {
    use serde::de::{Error, Unexpected};
    use std::borrow::Cow;

    let intermediate = <Option<Cow<'de, str>>>::deserialize(de)?;
    match intermediate.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => Url::parse(s).map_or_else(
            |err| {
                Err(Error::invalid_value(
                    Unexpected::Str(s),
                    &err.to_string().as_str(),
                ))
            },
            |ok| Ok(Some(ok)),
        ),
    }
}
