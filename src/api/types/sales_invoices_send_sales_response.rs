pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesSendSalesResponse {
    #[serde(default)]
    pub sent: bool,
    #[serde(default)]
    pub to: String,
}

impl InvoicesSendSalesResponse {
    pub fn builder() -> InvoicesSendSalesResponseBuilder {
        <InvoicesSendSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesSendSalesResponseBuilder {
    sent: Option<bool>,
    to: Option<String>,
}

impl InvoicesSendSalesResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesSendSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](InvoicesSendSalesResponseBuilder::sent)
    /// - [`to`](InvoicesSendSalesResponseBuilder::to)
    pub fn build(self) -> Result<InvoicesSendSalesResponse, BuildError> {
        Ok(InvoicesSendSalesResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
