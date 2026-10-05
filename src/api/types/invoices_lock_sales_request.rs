pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesLockSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesLockSalesRequest {
    pub fn builder() -> InvoicesLockSalesRequestBuilder {
        <InvoicesLockSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesLockSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesLockSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesLockSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesLockSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesLockSalesRequest, BuildError> {
        Ok(InvoicesLockSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
