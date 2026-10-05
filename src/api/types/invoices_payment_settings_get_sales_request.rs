pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPaymentSettingsGetSalesRequest {}

impl InvoicesPaymentSettingsGetSalesRequest {
    pub fn builder() -> InvoicesPaymentSettingsGetSalesRequestBuilder {
        <InvoicesPaymentSettingsGetSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentSettingsGetSalesRequestBuilder {}

impl InvoicesPaymentSettingsGetSalesRequestBuilder {
    /// Consumes the builder and constructs a [`InvoicesPaymentSettingsGetSalesRequest`].
    pub fn build(self) -> Result<InvoicesPaymentSettingsGetSalesRequest, BuildError> {
        Ok(InvoicesPaymentSettingsGetSalesRequest {})
    }
}
