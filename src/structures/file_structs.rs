use super::{
    common_structs::{ModLoaderType, SortableGameVersion},
    Datetime, ID,
};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// The file id
    pub id: ID,
    /// The game id related to the mod that this file belongs to
    pub game_id: ID,
    /// The mod id
    pub mod_id: ID,
    /// Whether the file is available to download
    pub is_available: bool,
    /// Display name of the file
    pub display_name: String,
    /// Exact file name
    pub file_name: String,
    /// The file release type
    pub release_type: FileReleaseType,
    /// Status of the file
    pub file_status: FileStatus,
    /// The file hash (i.e. md5 or sha1)
    pub hashes: Vec<FileHash>,
    /// The file timestamp
    pub file_date: Datetime,
    /// The file length in bytes
    pub file_length: u64,
    /// The number of downloads for the file
    pub download_count: u64,
    /// The file download URL
    pub download_url: String,
    /// List of game versions this file is relevant for
    pub game_versions: Vec<String>,
    /// Metadata used for sorting by game versions
    pub sortable_game_versions: Vec<SortableGameVersion>,
    /// List of dependencies files
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<ID>,
    pub alternate_file_id: Option<ID>,
    pub is_server_pack: Option<bool>,
    pub server_pack_file_id: Option<ID>,
    pub file_fingerprint: u64,
    pub modules: Vec<FileModule>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FileReleaseType {
    #[serde(rename = "1")]
    Release,
    #[serde(rename = "2")]
    Beta,
    #[serde(rename = "3")]
    Alpha,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FileStatus {
    #[serde(rename = "1")]
    Processing,
    #[serde(rename = "2")]
    ChangesRequired,
    #[serde(rename = "3")]
    UnderReview,
    #[serde(rename = "4")]
    Approved,
    #[serde(rename = "5")]
    Rejected,
    #[serde(rename = "6")]
    MalwareDetected,
    #[serde(rename = "7")]
    Deleted,
    #[serde(rename = "8")]
    Archived,
    #[serde(rename = "9")]
    Testing,
    #[serde(rename = "10")]
    Released,
    #[serde(rename = "11")]
    ReadyForReview,
    #[serde(rename = "12")]
    Deprecated,
    #[serde(rename = "13")]
    Baking,
    #[serde(rename = "14")]
    AwaitingPublishing,
    #[serde(rename = "15")]
    FailedPublishing,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: ID,
    pub filename: String,
    pub release_type: FileReleaseType,
    pub game_version_type_id: Option<ID>,
    pub mod_loader: Option<ModLoaderType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum HashAlgo {
    #[serde(rename = "1")]
    Sha1,
    #[serde(rename = "2")]
    Md5,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: ID,
    pub field: ID,
    pub relation_type: FileRelationType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FileRelationType {
    #[serde(rename = "1")]
    EmbeddedLibrary,
    #[serde(rename = "2")]
    OptionalDependency,
    #[serde(rename = "3")]
    RequiredDependency,
    #[serde(rename = "4")]
    Tool,
    #[serde(rename = "5")]
    Incompatible,
    #[serde(rename = "6")]
    Include,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: i64,
}
