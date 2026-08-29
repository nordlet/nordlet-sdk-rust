pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1BankFeedsConnectionsDeleteResponse {
    pub fn builder() -> PostV1BankFeedsConnectionsDeleteResponseBuilder {
        <PostV1BankFeedsConnectionsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1BankFeedsConnectionsDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1BankFeedsConnectionsDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsDeleteResponse, BuildError> {
        Ok(PostV1BankFeedsConnectionsDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
