pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPaymentSettingsUpdateSalesRequest {
    #[serde(rename = "paymentLinkTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_template: Option<String>,
}

impl InvoicesPaymentSettingsUpdateSalesRequest {
    pub fn builder() -> InvoicesPaymentSettingsUpdateSalesRequestBuilder {
        <InvoicesPaymentSettingsUpdateSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentSettingsUpdateSalesRequestBuilder {
    payment_link_template: Option<String>,
}

impl InvoicesPaymentSettingsUpdateSalesRequestBuilder {
    pub fn payment_link_template(mut self, value: impl Into<String>) -> Self {
        self.payment_link_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPaymentSettingsUpdateSalesRequest`].
    pub fn build(self) -> Result<InvoicesPaymentSettingsUpdateSalesRequest, BuildError> {
        Ok(InvoicesPaymentSettingsUpdateSalesRequest {
            payment_link_template: self.payment_link_template,
        })
    }
}
