pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesEinvoiceSendSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesEinvoiceSendSalesRequest {
    pub fn builder() -> InvoicesEinvoiceSendSalesRequestBuilder {
        <InvoicesEinvoiceSendSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesEinvoiceSendSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesEinvoiceSendSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesEinvoiceSendSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesEinvoiceSendSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesEinvoiceSendSalesRequest, BuildError> {
        Ok(InvoicesEinvoiceSendSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
