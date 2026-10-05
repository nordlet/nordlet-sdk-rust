pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesEinvoiceStatusSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesEinvoiceStatusSalesRequest {
    pub fn builder() -> InvoicesEinvoiceStatusSalesRequestBuilder {
        <InvoicesEinvoiceStatusSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesEinvoiceStatusSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesEinvoiceStatusSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesEinvoiceStatusSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesEinvoiceStatusSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesEinvoiceStatusSalesRequest, BuildError> {
        Ok(InvoicesEinvoiceStatusSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
