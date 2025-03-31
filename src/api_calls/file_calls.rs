use crate::{
    request::API_URL_BASE,
    structures::{file_structs::*, ID},
    Furse, Result,
};

impl Furse {
    /// Get all the files of mod with `mod_id`
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's files
    /// let terralith_files = curseforge.get_mod_files(513688).await?;
    /// // Check that the latest file is downloadable
    /// assert!(terralith_files[0].is_available);
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_mod_files(&self, mod_id: ID) -> Result<Vec<File>> {
        let mut url = API_URL_BASE
            .join("mods/")?
            .join(&(mod_id.to_string() + "/"))?
            .join("files")?;
        url.set_query(Some("pageSize=10000"));
        Ok(self.get(url).await?.data)
    }

    /// Get the file with `file_id` of mod with `mod_id`
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Check that it contains the version in the file name
    /// assert!(terralith_file.file_name.contains("v2.0.12"));
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_mod_file(&self, mod_id: ID, file_id: ID) -> Result<File> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("files/")?
                    .join(&file_id.to_string())?,
            )
            .await?
            .data)
    }

    /// Get the changelog of the file with `file_id` of mod with `mod_id` in HTML format
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file's changelog
    /// let changelog = curseforge.get_mod_file_changelog(513688, 3606078).await?;
    /// // This update had huge performance updates, so it should be mentioned in the changelog
    /// assert!(changelog.contains("performance"));
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_mod_file_changelog(&self, mod_id: ID, file_id: ID) -> Result<String> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("files/")?
                    .join(&(file_id.to_string() + "/"))?
                    .join("changelog")?,
            )
            .await?
            .data)
    }

    /// Get the download URL of the file with `file_id` of mod with `mod_id`
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get information about the file
    /// let terralith_mod_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Get the file's download url
    /// let download_url = curseforge.file_download_url(513688, 3606078).await?;
    /// // They should be the same url
    /// assert_eq!(Some(download_url), terralith_mod_file.download_url);
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn file_download_url(&self, mod_id: ID, file_id: ID) -> Result<url::Url> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&(mod_id.to_string() + "/"))?
                    .join("files/")?
                    .join(&(file_id.to_string() + "/"))?
                    .join("download-url")?,
            )
            .await?
            .data)
    }

    /// Get a list of files from the `file_ids` provided
    ///
    /// This function additionally sorts the returned files in the order you requested them in,
    /// and leaves `None` in places where a file was not found.
    ///
    /// ## Example
    /// ```rust
    /// # #![feature(assert_matches)]
    /// # use std::assert_matches::assert_matches;
    /// # use furse::structures::file_structs::File;
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Try getting 2 real files, and a non-existent one (1234)
    /// let files = curseforge.get_files(vec![3144153, 3778436, 1234]).await?;
    /// // The first two files should be `Some`,
    /// // and their IDs should be in the order we requested them in
    /// assert_matches!(files[0], Some(File { id: 3144153, .. }));
    /// assert_matches!(files[1], Some(File { id: 3778436, .. }));
    /// // But the last one should be `None` as it doesn't exist
    /// assert!(files[2].is_none());
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_files(&self, file_ids: Vec<ID>) -> Result<Vec<Option<File>>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GetFilesBodyRequestBody {
            file_ids: Vec<ID>,
        }

        let file_ids = GetFilesBodyRequestBody { file_ids };
        let mut files: Vec<File> = self
            .post(API_URL_BASE.join("mods/")?.join("files")?, &file_ids)
            .await?
            .data;
        let mut ordered_files = Vec::new();
        for file_id in file_ids.file_ids {
            if let Some(index) = files.iter().position(|file| file.id == file_id) {
                ordered_files.push(Some(files.swap_remove(index)));
            } else {
                ordered_files.push(None);
            }
        }
        Ok(ordered_files)
    }
}
