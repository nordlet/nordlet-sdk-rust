pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsSignaturesDeleteRequestBuilder {
        <PostV1DeclarationsAnnualAccountsSignaturesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsSignaturesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsSignaturesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsSignaturesDeleteRequestBuilder::id)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsSignaturesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
