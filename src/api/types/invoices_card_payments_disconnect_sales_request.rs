pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCardPaymentsDisconnectSalesRequest {}

impl InvoicesCardPaymentsDisconnectSalesRequest {
    pub fn builder() -> InvoicesCardPaymentsDisconnectSalesRequestBuilder {
        <InvoicesCardPaymentsDisconnectSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCardPaymentsDisconnectSalesRequestBuilder {}

impl InvoicesCardPaymentsDisconnectSalesRequestBuilder {
    /// Consumes the builder and constructs a [`InvoicesCardPaymentsDisconnectSalesRequest`].
    pub fn build(self) -> Result<InvoicesCardPaymentsDisconnectSalesRequest, BuildError> {
        Ok(InvoicesCardPaymentsDisconnectSalesRequest {})
    }
}
