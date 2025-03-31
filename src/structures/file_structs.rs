use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: ID,
    /// The game ID of the mod that this file belongs to
    pub game_id: ID,
    pub mod_id: ID,
    pub is_available: bool,
    pub display_name: String,
    /// Exact file name
    pub file_name: String,
    pub release_type: FileReleaseType,
    pub file_status: FileStatus,
    pub hashes: Vec<FileHash>,
    pub file_date: UtcTime,
    /// The file length in bytes
    pub file_length: Number,
    pub download_count: Number,
    pub file_size_on_disk: Option<Number>,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub download_url: Option<Url>,
    /// List of game versions this file is relevant for
    pub game_versions: Vec<String>,
    /// Metadata used for sorting by game versions
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<ID>,
    pub alternate_file_id: Option<ID>,
    pub is_server_pack: Option<bool>,
    pub server_pack_file_id: Option<ID>,
    pub is_early_access_content: Option<bool>,
    pub early_access_end_date: Option<UtcTime>,
    pub file_fingerprint: Number,
    pub modules: Vec<FileModule>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
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
    Cooking = 16,
    Cooked = 17,
    UnderManualReview = 18,
    ScanningForMalware = 19,
    ProcessingFile = 20,
    PendingRelease = 21,
    ReadyForCooking = 22,
    PostProcessing = 23,
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
#[serde(rename_all = "camelCase")]
pub struct FileHash {
    pub value: String,
    pub algo: HashAlgo,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: ID,
    pub relation_type: FileRelationType,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct FileModule {
    pub name: String,
    pub fingerprint: Number,
}
