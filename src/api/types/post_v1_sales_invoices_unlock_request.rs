pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesUnlockRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesUnlockRequest {
    pub fn builder() -> PostV1SalesInvoicesUnlockRequestBuilder {
        <PostV1SalesInvoicesUnlockRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesUnlockRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesUnlockRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesUnlockRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesUnlockRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesUnlockRequest, BuildError> {
        Ok(PostV1SalesInvoicesUnlockRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
