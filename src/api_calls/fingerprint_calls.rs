use crate::{request::API_URL_BASE, structures::fingerprint_structs::*, Furse, Result};
use murmur2::murmur2;

/// Calculate the CurseForge fingerprint for the `bytes` provided
///
/// CurseForge uses a modified version of murmur2 where some bytes are stripped,
/// and the resulting bytes are hashes with seed `1`
pub fn cf_fingerprint(bytes: &[u8]) -> usize {
    // Implement CF's murmur2 modification
    let bytes = bytes
        .iter()
        .filter(|x| !matches!(x, 9 | 10 | 13 | 32))
        .copied()
        .collect::<Vec<u8>>();
    // Hash the contents using seed `1`
    murmur2(&bytes, 1) as usize
}

impl Furse {
    /// Get file structs from the `fingerprints` provided.
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
    /// // Hash the contents
    /// let fingerprint = furse::cf_fingerprint(&contents);
    /// // Get the fingerprint matches
    /// let matches = curseforge.get_fingerprint_matches(vec![fingerprint]).await?.exact_matches;
    /// // The resulting file should have the same ID
    /// assert_eq!(matches[0].file.id, terralith_file.id);
    /// # Ok(()) }
    /// ```
    pub async fn get_fingerprint_matches(
        &self,
        fingerprints: Vec<usize>,
    ) -> Result<FingerprintMatches> {
        Ok(self
            .post(
                API_URL_BASE.join("fingerprints")?,
                &GetFingerprintMatchesBody { fingerprints },
            )
            .await?
            .data)
    }
}
