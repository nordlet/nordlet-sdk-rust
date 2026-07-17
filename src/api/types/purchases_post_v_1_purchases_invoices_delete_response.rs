pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1PurchasesInvoicesDeleteResponse {
    pub fn builder() -> PostV1PurchasesInvoicesDeleteResponseBuilder {
        <PostV1PurchasesInvoicesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1PurchasesInvoicesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesInvoicesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesDeleteResponse, BuildError> {
        Ok(PostV1PurchasesInvoicesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
