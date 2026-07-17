pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersBankAccountsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PartnersBankAccountsDeleteRequest {
    pub fn builder() -> PostV1PartnersBankAccountsDeleteRequestBuilder {
        <PostV1PartnersBankAccountsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersBankAccountsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PartnersBankAccountsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersBankAccountsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersBankAccountsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersBankAccountsDeleteRequest, BuildError> {
        Ok(PostV1PartnersBankAccountsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
