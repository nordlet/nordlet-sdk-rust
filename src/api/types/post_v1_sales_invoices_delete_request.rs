pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesDeleteRequest {
    pub fn builder() -> PostV1SalesInvoicesDeleteRequestBuilder {
        <PostV1SalesInvoicesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesDeleteRequest, BuildError> {
        Ok(PostV1SalesInvoicesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
