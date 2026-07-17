pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesInvoicesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1SalesInvoicesGetRequest {
    pub fn builder() -> PostV1SalesInvoicesGetRequestBuilder {
        <PostV1SalesInvoicesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesInvoicesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1SalesInvoicesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesInvoicesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesInvoicesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1SalesInvoicesGetRequest, BuildError> {
        Ok(PostV1SalesInvoicesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
