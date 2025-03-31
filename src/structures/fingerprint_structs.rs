use super::{file_structs::File, *};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintMatches {
    pub is_cache_built: bool,
    /// The matches found
    pub exact_matches: Vec<Match>,
    /// The fingerprints of the matches found
    pub exact_fingerprints: Vec<Number>,
    pub partial_matches: Vec<Match>,
    pub partial_match_fingerprints: std::collections::HashMap<String, Vec<Number>>,
    /// The fingerprints that were requested
    pub installed_fingerprints: Vec<Number>,
    pub unmatched_fingerprints: Option<Vec<Number>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    /// The ID of the mod
    pub id: ID,
    pub file: File,
    /// The mod's latest files
    pub latest_files: Vec<File>,
}
