pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesDeletePurchasesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesDeletePurchasesRequest {
    pub fn builder() -> InvoicesDeletePurchasesRequestBuilder {
        <InvoicesDeletePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesDeletePurchasesRequestBuilder {
    id: Option<String>,
}

impl InvoicesDeletePurchasesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesDeletePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesDeletePurchasesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesDeletePurchasesRequest, BuildError> {
        Ok(InvoicesDeletePurchasesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
