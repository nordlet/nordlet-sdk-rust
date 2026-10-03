pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse {
    pub fn builder() -> PostV1DeclarationsAnnualAccountsDistributionsDeleteResponseBuilder {
        <PostV1DeclarationsAnnualAccountsDistributionsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsAnnualAccountsDistributionsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsAnnualAccountsDistributionsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsAnnualAccountsDistributionsDeleteResponseBuilder::id)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse, BuildError> {
        Ok(
            PostV1DeclarationsAnnualAccountsDistributionsDeleteResponse {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            },
        )
    }
}
