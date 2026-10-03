pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCertificatesUploadResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsCertificatesUploadResponseRowsItem>,
}

impl PostV1DeclarationsCertificatesUploadResponse {
    pub fn builder() -> PostV1DeclarationsCertificatesUploadResponseBuilder {
        <PostV1DeclarationsCertificatesUploadResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCertificatesUploadResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsCertificatesUploadResponseRowsItem>>,
}

impl PostV1DeclarationsCertificatesUploadResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsCertificatesUploadResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCertificatesUploadResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsCertificatesUploadResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsCertificatesUploadResponse, BuildError> {
        Ok(PostV1DeclarationsCertificatesUploadResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
