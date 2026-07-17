pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesInvoicesGetRequest {
    pub fn builder() -> PostV1PurchasesInvoicesGetRequestBuilder {
        <PostV1PurchasesInvoicesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1PurchasesInvoicesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesInvoicesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesGetRequest, BuildError> {
        Ok(PostV1PurchasesInvoicesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
