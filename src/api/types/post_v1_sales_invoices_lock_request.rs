pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesLockRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesLockRequest {
    pub fn builder() -> PostV1SalesInvoicesLockRequestBuilder {
        <PostV1SalesInvoicesLockRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesLockRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesLockRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesLockRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesLockRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesLockRequest, BuildError> {
        Ok(PostV1SalesInvoicesLockRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
