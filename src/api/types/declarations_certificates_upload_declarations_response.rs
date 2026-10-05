pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CertificatesUploadDeclarationsResponse {
    #[serde(default)]
    pub rows: Vec<CertificatesUploadDeclarationsResponseRowsItem>,
}

impl CertificatesUploadDeclarationsResponse {
    pub fn builder() -> CertificatesUploadDeclarationsResponseBuilder {
        <CertificatesUploadDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesUploadDeclarationsResponseBuilder {
    rows: Option<Vec<CertificatesUploadDeclarationsResponseRowsItem>>,
}

impl CertificatesUploadDeclarationsResponseBuilder {
    pub fn rows(mut self, value: Vec<CertificatesUploadDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CertificatesUploadDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](CertificatesUploadDeclarationsResponseBuilder::rows)
    pub fn build(self) -> Result<CertificatesUploadDeclarationsResponse, BuildError> {
        Ok(CertificatesUploadDeclarationsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
