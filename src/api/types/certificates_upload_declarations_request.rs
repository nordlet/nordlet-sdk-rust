pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CertificatesUploadDeclarationsRequest {
    #[serde(default)]
    pub system: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    /// Base64-encoded PEM or PKCS#12 file
    #[serde(default)]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
}

impl CertificatesUploadDeclarationsRequest {
    pub fn builder() -> CertificatesUploadDeclarationsRequestBuilder {
        <CertificatesUploadDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificatesUploadDeclarationsRequestBuilder {
    system: Option<String>,
    file_name: Option<String>,
    content: Option<String>,
    passphrase: Option<String>,
}

impl CertificatesUploadDeclarationsRequestBuilder {
    pub fn system(mut self, value: impl Into<String>) -> Self {
        self.system = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn passphrase(mut self, value: impl Into<String>) -> Self {
        self.passphrase = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CertificatesUploadDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system`](CertificatesUploadDeclarationsRequestBuilder::system)
    /// - [`file_name`](CertificatesUploadDeclarationsRequestBuilder::file_name)
    /// - [`content`](CertificatesUploadDeclarationsRequestBuilder::content)
    pub fn build(self) -> Result<CertificatesUploadDeclarationsRequest, BuildError> {
        Ok(CertificatesUploadDeclarationsRequest {
            system: self
                .system
                .ok_or_else(|| BuildError::missing_field("system"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            passphrase: self.passphrase,
        })
    }
}
