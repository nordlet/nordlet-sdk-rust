pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerOwnersDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1LedgerOwnersDeleteRequest {
    pub fn builder() -> PostV1LedgerOwnersDeleteRequestBuilder {
        <PostV1LedgerOwnersDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerOwnersDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1LedgerOwnersDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerOwnersDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerOwnersDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1LedgerOwnersDeleteRequest, BuildError> {
        Ok(PostV1LedgerOwnersDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
