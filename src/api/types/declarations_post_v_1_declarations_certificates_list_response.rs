pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCertificatesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsCertificatesListResponseRowsItem>,
}

impl PostV1DeclarationsCertificatesListResponse {
    pub fn builder() -> PostV1DeclarationsCertificatesListResponseBuilder {
        <PostV1DeclarationsCertificatesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCertificatesListResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsCertificatesListResponseRowsItem>>,
}

impl PostV1DeclarationsCertificatesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1DeclarationsCertificatesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCertificatesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsCertificatesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsCertificatesListResponse, BuildError> {
        Ok(PostV1DeclarationsCertificatesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
