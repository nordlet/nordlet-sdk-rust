pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCertificatesDeleteResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsCertificatesDeleteResponseRowsItem>,
}

impl PostV1DeclarationsCertificatesDeleteResponse {
    pub fn builder() -> PostV1DeclarationsCertificatesDeleteResponseBuilder {
        <PostV1DeclarationsCertificatesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCertificatesDeleteResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsCertificatesDeleteResponseRowsItem>>,
}

impl PostV1DeclarationsCertificatesDeleteResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsCertificatesDeleteResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsCertificatesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsCertificatesDeleteResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsCertificatesDeleteResponse, BuildError> {
        Ok(PostV1DeclarationsCertificatesDeleteResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
