pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCardPaymentsGetSalesResponse {
    #[serde(default)]
    pub connected: bool,
    #[serde(rename = "chargesEnabled")]
    #[serde(default)]
    pub charges_enabled: bool,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl InvoicesCardPaymentsGetSalesResponse {
    pub fn builder() -> InvoicesCardPaymentsGetSalesResponseBuilder {
        <InvoicesCardPaymentsGetSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCardPaymentsGetSalesResponseBuilder {
    connected: Option<bool>,
    charges_enabled: Option<bool>,
    account_id: Option<String>,
}

impl InvoicesCardPaymentsGetSalesResponseBuilder {
    pub fn connected(mut self, value: bool) -> Self {
        self.connected = Some(value);
        self
    }

    pub fn charges_enabled(mut self, value: bool) -> Self {
        self.charges_enabled = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCardPaymentsGetSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connected`](InvoicesCardPaymentsGetSalesResponseBuilder::connected)
    /// - [`charges_enabled`](InvoicesCardPaymentsGetSalesResponseBuilder::charges_enabled)
    pub fn build(self) -> Result<InvoicesCardPaymentsGetSalesResponse, BuildError> {
        Ok(InvoicesCardPaymentsGetSalesResponse {
            connected: self
                .connected
                .ok_or_else(|| BuildError::missing_field("connected"))?,
            charges_enabled: self
                .charges_enabled
                .ok_or_else(|| BuildError::missing_field("charges_enabled"))?,
            account_id: self.account_id,
        })
    }
}
