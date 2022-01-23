use crate::{
    api_calls::Response,
    request::{request, request_rel},
    structures::{file_structs::File, ID},
    Furse, Result,
};
use bytes::Bytes;

impl Furse {
    /// Get the files of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's files
    /// let terralith_files = curseforge.get_mod_files(513688).await?;
    /// // Check that the latest file is downloadable
    /// assert!(terralith_files[0].is_available);
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn get_mod_files(&self, mod_id: ID) -> Result<Vec<File>> {
        let response: Response<Vec<File>> =
            request_rel(self, format!("/mods/{}/files?pageSize=10000", mod_id))
                .await?
                .json()
                .await?;
        Ok(response.data)
    }

    /// Get the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Check that it contains the version in the file name
    /// assert!(terralith_file.file_name.contains("v2.0.12"));
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn get_mod_file(&self, mod_id: ID, file_id: ID) -> Result<File> {
        let response: Response<File> =
            request_rel(self, format!("/mods/{}/files/{}", mod_id, file_id))
                .await?
                .json()
                .await?;
        Ok(response.data)
    }

    /// Get the changelog of the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file's changelog
    /// let terralith_file_changelog = curseforge.get_mod_file_changelog(513688, 3606078).await?;
    /// // This update had huge performance updates so check that that is mentioned in the changelog
    /// assert!(terralith_file_changelog.contains("performance"));
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn get_mod_file_changelog(&self, mod_id: ID, file_id: ID) -> Result<String> {
        let response: Response<String> = request_rel(
            self,
            format!("/mods/{}/files/{}/changelog", mod_id, file_id),
        )
        .await?
        .json()
        .await?;
        Ok(response.data)
    }

    /// Download and return `file`'s contents
    ///
    /// Example:
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_mod_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Download that file
    /// let contents = curseforge.download_mod_file_from_file(&terralith_mod_file).await?;
    /// // The contents returned should be of the length specified in the file struct
    /// assert!(contents.len() as u64 == terralith_mod_file.file_length);
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn download_mod_file_from_file(&self, file: &File) -> Result<Bytes> {
        request(self, &file.download_url).await?.bytes().await
    }

    /// Download and return contents of file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # tokio_test::block_on( async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Download the Terralith mod's v2.0.12 file
    /// let contents = curseforge.download_mod_file_from_file_id(513688, 3606078).await?;
    /// // Get information about the file
    /// let terralith_mod_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // The contents returned should be of the length specified in the file struct
    /// assert!(contents.len() as u64 == terralith_mod_file.file_length);
    /// # Ok::<(), reqwest::Error>(()) } );
    /// ```
    pub async fn download_mod_file_from_file_id(&self, mod_id: ID, file_id: ID) -> Result<Bytes> {
        let response: Response<String> = request_rel(
            self,
            format!("/mods/{}/files/{}/download-url", mod_id, file_id),
        )
        .await?
        .json()
        .await?;
        request(self, response.data).await?.bytes().await
    }
}
