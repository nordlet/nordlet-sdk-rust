pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponseBuilder {
        <PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponseBuilder::id)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsAttachmentsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
