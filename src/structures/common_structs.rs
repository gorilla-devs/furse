use super::{Datetime, ID};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// The category id
    pub id: ID,
    /// The game id related to the category
    pub game_id: ID,
    /// Category name
    pub name: String,
    /// The category slug as it appear in the URL
    pub slug: String,
    /// The category URL
    pub url: String,
    /// URL for the category icon
    pub icon_url: String,
    /// Last modified date of the category
    pub date_modified: Datetime,
    /// A top level category for other categories
    pub is_class: Option<bool>,
    /// The class id of the category, meaning - the class of which this category is under
    pub class_id: Option<ID>,
    /// The parent category for this category
    pub parent_category_id: Option<ID>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    /// Original version name (e.g. 1.5b)
    pub game_version_name: String,
    /// Used for sorting (e.g. 0000000001.0000000005)
    pub game_version_padded: String,
    /// game version clean name (e.g. 1.5)
    pub game_version: String,
    /// Game version release date
    pub game_version_release_date: Datetime,
    /// Game version type id
    pub game_version_type_id: Option<ID>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// A zero based index of the first item that is included in the response
    pub index: u32,
    /// The requested number of items to be included in the response
    pub page_size: u32,
    /// The actual number of items that were included in the response
    pub result_count: u32,
    /// The total number of items available in the request
    pub total_count: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ModLoaderType {
    #[serde(rename = "0")]
    Any,
    #[serde(rename = "1")]
    Forge,
    #[serde(rename = "2")]
    Cauldron,
    #[serde(rename = "3")]
    LiteLoader,
    #[serde(rename = "4")]
    Fabric,
}
