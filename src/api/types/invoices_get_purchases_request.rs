pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesGetPurchasesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesGetPurchasesRequest {
    pub fn builder() -> InvoicesGetPurchasesRequestBuilder {
        <InvoicesGetPurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesGetPurchasesRequestBuilder {
    id: Option<String>,
}

impl InvoicesGetPurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesGetPurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesGetPurchasesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesGetPurchasesRequest, BuildError> {
        Ok(InvoicesGetPurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
