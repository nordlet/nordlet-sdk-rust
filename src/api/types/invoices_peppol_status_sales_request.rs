pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPeppolStatusSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesPeppolStatusSalesRequest {
    pub fn builder() -> InvoicesPeppolStatusSalesRequestBuilder {
        <InvoicesPeppolStatusSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPeppolStatusSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesPeppolStatusSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPeppolStatusSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesPeppolStatusSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesPeppolStatusSalesRequest, BuildError> {
        Ok(InvoicesPeppolStatusSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
