use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: ID,
    /// The game ID related to the category
    pub game_id: ID,
    pub name: String,
    /// The category slug as it appear in the URL
    pub slug: String,
    pub url: Url,
    /// URL for the category icon
    pub icon_url: Url,
    /// Last modified date of the category
    pub date_modified: UtcTime,
    /// Whether this is a top level category for other categories
    pub is_class: Option<bool>,
    /// The ID of the class which this category is under
    pub class_id: Option<ID>,
    pub parent_category_id: Option<ID>,
    pub display_index: Option<Number>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    /// Original version name (e.g. 1.5b)
    pub game_version_name: String,
    /// Padded version used for sorting (e.g. 0000000001.0000000005)
    pub game_version_padded: String,
    /// Clean version (e.g. 1.5)
    pub game_version: String,
    pub game_version_release_date: UtcTime,
    pub game_version_type_id: Option<ID>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// The index of the first item included in the response
    pub index: Number,
    /// The requested number of items to be included in the response
    pub page_size: Number,
    /// The actual number of items that were included in the response
    pub result_count: Number,
    /// The total number of items available
    pub total_count: Number,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
}
