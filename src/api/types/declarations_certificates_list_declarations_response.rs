pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CertificatesListDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<CertificatesListDeclarationsResponseRowsItem>,
}

impl CertificatesListDeclarationsResponse {
    pub fn builder() -> CertificatesListDeclarationsResponseBuilder {
        <CertificatesListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesListDeclarationsResponseBuilder {
    rows: Option<Vec<CertificatesListDeclarationsResponseRowsItem>>,
}

impl CertificatesListDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<CertificatesListDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CertificatesListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](CertificatesListDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<CertificatesListDeclarationsResponse, BuildError> {
        Ok(CertificatesListDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
