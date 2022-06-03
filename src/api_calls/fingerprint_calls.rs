use crate::{
  request::API_URL_BASE,
  structures::{
      fingerprint_structs::{FingerprintsApiResponse, GetFingerprintMatchesBody},
  },
  Furse, Result,
};
impl Furse {
    pub async fn get_fingerprint_matches(&self, fingerprints: Vec<u32>) -> Result<FingerprintsApiResponse> {
        let body = GetFingerprintMatchesBody { fingerprints };
        Ok(self.post(API_URL_BASE.join("fingerprints")?, &body).await?.data)
    }
}