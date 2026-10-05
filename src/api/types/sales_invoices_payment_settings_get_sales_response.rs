pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPaymentSettingsGetSalesResponse {
    #[serde(rename = "paymentLinkTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_template: Option<String>,
}

impl InvoicesPaymentSettingsGetSalesResponse {
    pub fn builder() -> InvoicesPaymentSettingsGetSalesResponseBuilder {
        <InvoicesPaymentSettingsGetSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentSettingsGetSalesResponseBuilder {
    payment_link_template: Option<String>,
}

impl InvoicesPaymentSettingsGetSalesResponseBuilder {
    pub fn payment_link_template(mut self, value: impl Into<String>) -> Self {
        self.payment_link_template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPaymentSettingsGetSalesResponse`].
    pub fn build(self) -> Result<InvoicesPaymentSettingsGetSalesResponse, BuildError> {
        Ok(InvoicesPaymentSettingsGetSalesResponse {
            payment_link_template: self.payment_link_template,
        })
    }
}
