use crate::{
    api_calls::Response,
    request::{request, API_URL_BASE},
    structures::{file_structs::File, ID},
    Furse, Result,
};
use bytes::Bytes;

impl Furse {
    /// Get the files of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's files
    /// let terralith_files = curseforge.get_mod_files(513688).await?;
    /// // Check that the latest file is downloadable
    /// assert!(terralith_files[0].is_available);
    /// # Ok(()) }
    /// ```
    pub async fn get_mod_files(&self, mod_id: ID) -> Result<Vec<File>> {
        // "/mods/{}/files?pageSize=10000"
        let mut url = API_URL_BASE
            .join("mods/")?
            .join(&format!("{}/", mod_id))?
            .join("files")?;
        url.set_query(Some("pageSize=10000"));
        Ok(request(self, url)
            .await?
            .json::<Response<Vec<File>>>()
            .await?
            .data)
    }

    /// Get the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Check that it contains the version in the file name
    /// assert!(terralith_file.file_name.contains("v2.0.12"));
    /// # Ok(()) }
    /// ```
    pub async fn get_mod_file(&self, mod_id: ID, file_id: ID) -> Result<File> {
        Ok(request(
            self,
            API_URL_BASE
                .join("mods/")?
                .join(&format!("{}/", mod_id))?
                .join("files/")?
                .join(&file_id.to_string())?,
        )
        .await?
        .json::<Response<File>>()
        .await?
        .data)
    }

    /// Get the changelog of the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file's changelog
    /// let terralith_file_changelog = curseforge.get_mod_file_changelog(513688, 3606078).await?;
    /// // This update had huge performance updates so check that that is mentioned in the changelog
    /// assert!(terralith_file_changelog.contains("performance"));
    /// # Ok(()) }
    /// ```
    pub async fn get_mod_file_changelog(&self, mod_id: ID, file_id: ID) -> Result<String> {
        Ok(request(
            self,
            API_URL_BASE
                .join("mods/")?
                .join(&format!("{}/", mod_id))?
                .join("files/")?
                .join(&format!("{}/", file_id))?
                .join("changelog")?,
        )
        .await?
        .json::<Response<String>>()
        .await?
        .data)
    }

    /// Download and return `file`'s contents
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_mod_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Download that file
    /// let contents = curseforge.download_mod_file_from_file(&terralith_mod_file).await?;
    /// // The contents returned should be of the length specified in the file struct
    /// assert!(contents.len() as u64 == terralith_mod_file.file_length);
    /// # Ok(()) }
    /// ```
    pub async fn download_mod_file_from_file(&self, file: &File) -> Result<Bytes> {
        Ok(request(self, &file.download_url).await?.bytes().await?)
    }

    /// Download and return contents of file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Download the Terralith mod's v2.0.12 file
    /// let contents = curseforge.download_mod_file_from_file_id(513688, 3606078).await?;
    /// // Get information about the file
    /// let terralith_mod_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // The contents returned should be of the length specified in the file struct
    /// assert!(contents.len() as u64 == terralith_mod_file.file_length);
    /// # Ok(()) }
    /// ```
    pub async fn download_mod_file_from_file_id(&self, mod_id: ID, file_id: ID) -> Result<Bytes> {
        let response: Response<String> = request(
            self,
            API_URL_BASE
                .join("mods/")?
                .join(&format!("{}/", mod_id))?
                .join("files/")?
                .join(&format!("{}/", file_id))?
                .join("download-url")?,
        )
        .await?
        .json()
        .await?;
        Ok(request(self, response.data).await?.bytes().await?)
    }
}
