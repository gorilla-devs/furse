use crate::{
    request::request_rel,
    structures::{mod_structs::Mod, ID},
    Furse, Result,
};

use super::Response;

impl Furse {
    /// Get mod with ID `mod_id`
    ///
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod
    /// let terralith_mod = curseforge.get_mod(513688).await?;
    /// // Check that it is made by Starmute
    /// assert!(terralith_mod.authors[0].name == "Starmute");
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn get_mod(&self, mod_id: ID) -> Result<Mod> {
        let response: Response<Mod> = request_rel(self, format!("/mods/{}", mod_id))
            .await?
            .json()
            .await?;
        Ok(response.data)
    }

    /// Get the description of mod with ID `mod_id`
    ///
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's description
    /// let terralith_mod_description = curseforge.get_mod_description(513688).await?;
    /// // The description would obviously contains the mod's name
    /// assert!(terralith_mod_description.contains("Terralith"));
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn get_mod_description(&self, mod_id: ID) -> Result<String> {
        let response: Response<String> = request_rel(self, format!("/mods/{}/description", mod_id))
            .await?
            .json()
            .await?;
        Ok(response.data)
    }
}
