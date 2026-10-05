pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CertificatesDeleteDeclarationsRequest {
    #[serde(default)]
    pub system: String,
    #[serde(rename = "fieldKey")]
    pub field_key: CertificatesDeleteDeclarationsRequestFieldKey,
}

impl CertificatesDeleteDeclarationsRequest {
    pub fn builder() -> CertificatesDeleteDeclarationsRequestBuilder {
        <CertificatesDeleteDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesDeleteDeclarationsRequestBuilder {
    system: Option<String>,
    field_key: Option<CertificatesDeleteDeclarationsRequestFieldKey>,
}

impl CertificatesDeleteDeclarationsRequestBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn field_key(mut self, value: CertificatesDeleteDeclarationsRequestFieldKey) -> Self {
        self.field_key = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CertificatesDeleteDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](CertificatesDeleteDeclarationsRequestBuilder::system)
    /// - [`field_key`](CertificatesDeleteDeclarationsRequestBuilder::field_key)
    pub fn build(self) -> Result<CertificatesDeleteDeclarationsRequest, BuildError> {
        Ok(CertificatesDeleteDeclarationsRequest {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            field_key: self
                .field_key
                .ok_or_else(|| BuildError::missing_field("field_key"))?,
        })
    }
}
