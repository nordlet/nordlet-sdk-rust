pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesUnlockSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesUnlockSalesRequest {
    pub fn builder() -> InvoicesUnlockSalesRequestBuilder {
        <InvoicesUnlockSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesUnlockSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesUnlockSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesUnlockSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesUnlockSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesUnlockSalesRequest, BuildError> {
        Ok(InvoicesUnlockSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
