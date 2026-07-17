pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersBankAccountsDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1PartnersBankAccountsDeleteResponse {
    pub fn builder() -> PostV1PartnersBankAccountsDeleteResponseBuilder {
        <PostV1PartnersBankAccountsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersBankAccountsDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1PartnersBankAccountsDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersBankAccountsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1PartnersBankAccountsDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1PartnersBankAccountsDeleteResponse, BuildError> {
        Ok(PostV1PartnersBankAccountsDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
