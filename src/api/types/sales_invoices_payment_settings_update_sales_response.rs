pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPaymentSettingsUpdateSalesResponse {
    #[serde(rename = "paymentLinkTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_template: Option<String>,
}

impl InvoicesPaymentSettingsUpdateSalesResponse {
    pub fn builder() -> InvoicesPaymentSettingsUpdateSalesResponseBuilder {
        <InvoicesPaymentSettingsUpdateSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentSettingsUpdateSalesResponseBuilder {
    payment_link_template: Option<String>,
}

impl InvoicesPaymentSettingsUpdateSalesResponseBuilder {
    pub fn payment_link_template(mut self, value: impl Into<String>) -> Self {
        self.payment_link_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPaymentSettingsUpdateSalesResponse`].
    pub fn build(self) -> Result<InvoicesPaymentSettingsUpdateSalesResponse, BuildError> {
        Ok(InvoicesPaymentSettingsUpdateSalesResponse {
            payment_link_template: self.payment_link_template,
        })
    }
}
