use super::{
    common_structs::{ModLoaderType, SortableGameVersion},
    *,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::clone::Clone;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
    pub file_length: Number,
    /// The number of downloads for the file
    pub download_count: Number,
    /// The file download URL.
    /// Is null if the mod has disabled mod distribution
    pub download_url: Option<String>,
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
    pub file_fingerprint: Number,
    pub modules: Vec<FileModule>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: ID,
    pub filename: String,
    pub release_type: FileReleaseType,
    pub game_version_type_id: Option<ID>,
    pub mod_loader: Option<ModLoaderType>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgo,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FileDependency {
    pub mod_id: ID,
    pub relation_type: FileRelationType,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: Number,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GetFilesBody {
    pub file_ids: Vec<ID>,
}
