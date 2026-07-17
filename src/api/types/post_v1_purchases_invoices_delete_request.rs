pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesInvoicesDeleteRequest {
    pub fn builder() -> PostV1PurchasesInvoicesDeleteRequestBuilder {
        <PostV1PurchasesInvoicesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1PurchasesInvoicesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesInvoicesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesDeleteRequest, BuildError> {
        Ok(PostV1PurchasesInvoicesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
