use super::{
    common_structs::Category,
    file_structs::{File, FileIndex},
    Datetime, ID,
};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    /// The mod ID
    pub id: ID,
    /// The game ID this mod is for
    pub game_id: ID,
    /// The name of the mod
    pub name: String,
    /// The mod slug that would appear in the URL
    pub slug: String,
    /// Relevant links for the mod such as Issue tracker and Wiki
    pub links: ModLinks,
    /// Mod summary
    pub summary: String,
    /// Current mod status
    pub status: ModStatus,
    /// Number of downloads for the mod
    pub download_count: usize,
    /// Whether the mod is included in the featured mods list
    pub is_featured: bool,
    /// The main category of the mod as it was chosen by the mod author
    pub primary_category_id: ID,
    /// List of categories that this mod is related to
    pub categories: Vec<Category>,
    /// List of the mod's authors
    pub authors: Vec<ModAuthor>,
    /// The mod's logo asset
    pub logo: ModAsset,
    /// List of screenshot assets
    pub screenshots: Vec<ModAsset>,
    /// The ID of the main file of the mod
    pub main_file_id: ID,
    /// List of latest files of the mod
    pub latest_files: Vec<File>,
    /// List of file related details for the latest files of the mod
    pub latest_files_indexes: Vec<FileIndex>,
    /// The creation date of the mod
    pub date_created: Datetime,
    /// The last time the mod was modified
    pub date_modified: Datetime,
    /// The release date of the mod
    pub date_released: Datetime,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    pub website_url: String,
    pub wiki_url: String,
    pub issues_url: String,
    pub source_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ModStatus {
    #[serde(rename = "1")]
    New,
    #[serde(rename = "2")]
    ChangesRequired,
    #[serde(rename = "3")]
    UnderSoftReview,
    #[serde(rename = "4")]
    Approved,
    #[serde(rename = "5")]
    Rejected,
    #[serde(rename = "6")]
    ChangesMade,
    #[serde(rename = "7")]
    Inactive,
    #[serde(rename = "8")]
    Abandoned,
    #[serde(rename = "9")]
    Deleted,
    #[serde(rename = "10")]
    UnderReview,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ModAuthor {
    pub id: ID,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModAsset {
    pub id: ID,
    pub mod_id: ID,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub url: String,
}
