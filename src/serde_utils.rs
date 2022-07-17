use serde::{de::Error, Deserializer};
use std::str::FromStr;

pub fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    if string.is_empty() {
        Ok(None)
    } else {
        let url = Url::from_str(&string).map_err(|e| Error::custom(e.to_string()))?;

        Ok(Some(url))
    }
}
