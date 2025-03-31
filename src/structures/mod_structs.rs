use super::{
    common_structs::Category,
    file_structs::{File, FileIndex},
    *,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub id: ID,
    pub game_id: ID,
    pub name: String,
    /// The mod slug that would appear in the URL
    pub slug: String,
    /// Relevant links for the mod such as the issue tracker and wiki
    pub links: ModLinks,
    pub summary: String,
    pub status: ModStatus,
    pub download_count: Number,
    /// Whether the mod is included in the featured mods list
    pub is_featured: bool,
    /// The main category of the mod as it was chosen by the mod author
    pub primary_category_id: ID,
    /// List of categories that this mod is related to
    pub categories: Vec<Category>,
    /// The ID of the class this mod belongs to
    pub class_id: Option<ID>,
    pub authors: Vec<ModAuthor>,
    pub logo: Option<ModAsset>,
    pub screenshots: Vec<ModAsset>,
    pub main_file_id: ID,
    pub latest_files: Vec<File>,
    /// List of file related details for the latest files of the mod
    pub latest_files_indexes: Vec<FileIndex>,
    pub latest_early_access_files_indexes: Vec<FileIndex>,
    pub date_created: UtcTime,
    pub date_modified: UtcTime,
    pub date_released: UtcTime,
    pub allow_mod_distribution: Option<bool>,
    pub game_popularity_rank: Number,
    /// Is the mod available for search.
    /// This can be false when a mod is experimental, in a deleted state, or has only alpha files.
    pub is_available: bool,
    pub thumbs_up_count: Number,
    pub rating: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    /// A link to the mod's CurseForge page
    pub website_url: Url,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub wiki_url: Option<Url>,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub issues_url: Option<Url>,
    #[serde(deserialize_with = "deserialise_optional_url")]
    pub source_url: Option<Url>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModAuthor {
    pub id: ID,
    pub name: String,
    pub url: Url,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModAsset {
    pub id: ID,
    pub mod_id: ID,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub url: Url,
}
