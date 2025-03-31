use super::*;
use mod_structs::*;

impl Furse {
    /// Get mod with ID `mod_id`
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod
    /// let terralith_mod = curseforge.get_mod(513688).await?;
    /// // Check that it is made by Starmute
    /// assert_eq!(terralith_mod.authors[0].name, "Starmute");
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_mod(&self, mod_id: ID) -> Result<Mod> {
        Ok(self
            .get(API_URL_BASE.join("mods/")?.join(&mod_id.to_string())?)
            .await?
            .data)
    }

    /// Get multiple mods with IDs `mod_ids`
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get Xaero's Minimap and World Map mods
    /// let mods = curseforge.get_mods(vec![263420, 317780]).await?;
    /// let [minimap, worldmap, ..] = mods.as_slice() else {
    ///     panic!("Expected 2 mods, got less");
    /// };
    /// // Check that both are made by `xaero96`
    /// assert_eq!(minimap.authors[0].name, "xaero96");
    /// assert_eq!(worldmap.authors[0].name, "xaero96");
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_mods(&self, mod_ids: Vec<ID>) -> Result<Vec<Mod>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GetModsByIdsListRequestBody {
            mod_ids: Vec<ID>,
        }
        Ok(self
            .post(
                API_URL_BASE.join("mods")?,
                &GetModsByIdsListRequestBody { mod_ids },
            )
            .await?
            .data)
    }

    /// Get the description of mod with ID `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's description
    /// let terralith_mod_description = curseforge.get_mod_description(513688).await?;
    /// // The description would obviously contains the mod's name
    /// assert!(terralith_mod_description.contains("Terralith"));
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_mod_description(&self, mod_id: ID) -> Result<String> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&format!("{}/", mod_id))?
                    .join("description")?,
            )
            .await?
            .data)
    }
}
