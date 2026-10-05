pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPaymentLinkSalesRequest {
    #[serde(default)]
    pub id: String,
}

impl InvoicesPaymentLinkSalesRequest {
    pub fn builder() -> InvoicesPaymentLinkSalesRequestBuilder {
        <InvoicesPaymentLinkSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentLinkSalesRequestBuilder {
    id: Option<String>,
}

impl InvoicesPaymentLinkSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPaymentLinkSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesPaymentLinkSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesPaymentLinkSalesRequest, BuildError> {
        Ok(InvoicesPaymentLinkSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
