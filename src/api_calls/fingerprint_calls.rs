use crate::{
    request::API_URL_BASE,
    structures::fingerprint_structs::{FingerprintMatches, GetFingerprintMatchesBody},
    Furse, Result,
};
use bytes::Bytes;
use murmur2::murmur2;

impl Furse {
    /// Get file structs from the `file_contents` provided.
    /// The fingerprints will be calculated by this function because they are a
    /// modified version of murmur2 which is not documented by CurseForge
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Download the file contents
    /// let mut contents = reqwest::get(terralith_file.download_url.unwrap())
    ///     .await?
    ///     .bytes()
    ///     .await?;
    /// // Get the fingerprint matches
    /// let matches = curseforge.get_fingerprint_matches(vec![contents]).await?.exact_matches;
    /// // The resulting file should have the same ID
    /// assert!(matches[0].file.id == terralith_file.id);
    /// # Ok(()) }
    /// ```
    pub async fn get_fingerprint_matches(
        &self,
        file_contents: Vec<Bytes>,
    ) -> Result<FingerprintMatches> {
        let mut fingerprints = Vec::new();
        for contents in file_contents {
            // Implement CF's murmur2 modification
            let contents = contents
                .into_iter()
                .filter(|&x| !matches!(x, 9 | 10 | 13 | 32))
                .collect::<Bytes>();
            // Hash the contents using seed `1`
            fingerprints.push(murmur2(&contents, 1) as usize);
        }
        Ok(self
            .post(
                API_URL_BASE.join("fingerprints")?,
                &GetFingerprintMatchesBody { fingerprints },
            )
            .await?
            .data)
    }
}
