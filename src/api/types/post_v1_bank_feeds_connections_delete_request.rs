pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankFeedsConnectionsDeleteRequest {
    pub fn builder() -> PostV1BankFeedsConnectionsDeleteRequestBuilder {
        <PostV1BankFeedsConnectionsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1BankFeedsConnectionsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankFeedsConnectionsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsDeleteRequest, BuildError> {
        Ok(PostV1BankFeedsConnectionsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
