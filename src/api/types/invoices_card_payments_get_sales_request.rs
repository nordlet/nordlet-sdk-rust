pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCardPaymentsGetSalesRequest {}

impl InvoicesCardPaymentsGetSalesRequest {
    pub fn builder() -> InvoicesCardPaymentsGetSalesRequestBuilder {
        <InvoicesCardPaymentsGetSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCardPaymentsGetSalesRequestBuilder {}

impl InvoicesCardPaymentsGetSalesRequestBuilder {
    /// Consumes the builder and constructs a [`InvoicesCardPaymentsGetSalesRequest`].
    pub fn build(self) -> Result<InvoicesCardPaymentsGetSalesRequest, BuildError> {
        Ok(InvoicesCardPaymentsGetSalesRequest {})
    }
}
