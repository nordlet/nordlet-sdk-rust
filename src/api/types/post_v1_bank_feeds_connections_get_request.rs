pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsConnectionsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankFeedsConnectionsGetRequest {
    pub fn builder() -> PostV1BankFeedsConnectionsGetRequestBuilder {
        <PostV1BankFeedsConnectionsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsConnectionsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1BankFeedsConnectionsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsConnectionsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankFeedsConnectionsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankFeedsConnectionsGetRequest, BuildError> {
        Ok(PostV1BankFeedsConnectionsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
