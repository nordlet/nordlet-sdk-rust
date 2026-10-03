pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsDistributionsDeleteRequestBuilder {
        <PostV1DeclarationsAnnualAccountsDistributionsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsDistributionsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsDistributionsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsDistributionsDeleteRequestBuilder::id)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest, BuildError> {
        Ok(PostV1DeclarationsAnnualAccountsDistributionsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
