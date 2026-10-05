pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPeppolSendSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesPeppolSendSalesRequest {
    pub fn builder() -> InvoicesPeppolSendSalesRequestBuilder {
        <InvoicesPeppolSendSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPeppolSendSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesPeppolSendSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPeppolSendSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesPeppolSendSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesPeppolSendSalesRequest, BuildError> {
        Ok(InvoicesPeppolSendSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
