use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::structures::{file_structs::File, ID};

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FingerprintsApiResponse {
    pub is_cache_built: bool,
    pub exact_matches: Vec<Match>,
    pub exact_fingerprints: Vec<u32>,
    pub partial_matches: Vec<Match>,
    pub partial_match_fingerprints: HashMap<String, Vec<u32>>,
    pub installed_fingerprints: Vec<u32>,
    // always seems to be null
    pub unmatched_fingerprints: Option<Vec<u32>>,
}
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Match {
    #[serde(rename = "id")]
    pub project_id: ID,
    pub file: File,
    pub latest_files: Vec<File>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GetFingerprintMatchesBody {
    pub fingerprints: Vec<u32>,
}
