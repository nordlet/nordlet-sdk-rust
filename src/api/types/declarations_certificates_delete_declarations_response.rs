pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CertificatesDeleteDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<CertificatesDeleteDeclarationsResponseRowsItem>,
}

impl CertificatesDeleteDeclarationsResponse {
    pub fn builder() -> CertificatesDeleteDeclarationsResponseBuilder {
        <CertificatesDeleteDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesDeleteDeclarationsResponseBuilder {
    rows: Option<Vec<CertificatesDeleteDeclarationsResponseRowsItem>>,
}

impl CertificatesDeleteDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<CertificatesDeleteDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CertificatesDeleteDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](CertificatesDeleteDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<CertificatesDeleteDeclarationsResponse, BuildError> {
        Ok(CertificatesDeleteDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
