use super::*;
use crate::structures::fingerprint_structs::*;

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
    murmur2::murmur2(&bytes, 1) as usize
}

impl Furse {
    /// Get file structs from the `fingerprints` provided
    ///
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_file = curseforge.get_mod_file(513688, 3606078).await?;
    /// // Download the file contents
    /// let contents = reqwest::get(terralith_file.download_url.unwrap())
    ///     .await?
    ///     .bytes()
    ///     .await?;
    /// // Hash the contents
    /// let fingerprint = furse::cf_fingerprint(&contents);
    /// // Get the fingerprint matches
    /// let matches = curseforge.get_fingerprint_matches(vec![fingerprint])
    ///     .await?
    ///     .exact_matches;
    /// // The resulting file should have the same ID
    /// assert_eq!(matches[0].file.id, terralith_file.id);
    /// # Ok::<_, furse::Error>(()) }).unwrap()
    /// ```
    pub async fn get_fingerprint_matches(
        &self,
        fingerprints: Vec<usize>,
    ) -> Result<FingerprintMatches> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GetFingerprintMatchesBody {
            fingerprints: Vec<usize>,
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
