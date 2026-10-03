pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequestBuilder {
        <PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequestBuilder::id)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsAttachmentsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
