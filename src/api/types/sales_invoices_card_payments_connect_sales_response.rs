pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCardPaymentsConnectSalesResponse {
    #[serde(default)]
    pub url: String,
}

impl InvoicesCardPaymentsConnectSalesResponse {
    pub fn builder() -> InvoicesCardPaymentsConnectSalesResponseBuilder {
        <InvoicesCardPaymentsConnectSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCardPaymentsConnectSalesResponseBuilder {
    url: Option<String>,
}

impl InvoicesCardPaymentsConnectSalesResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCardPaymentsConnectSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](InvoicesCardPaymentsConnectSalesResponseBuilder::url)
    pub fn build(self) -> Result<InvoicesCardPaymentsConnectSalesResponse, BuildError> {
        Ok(InvoicesCardPaymentsConnectSalesResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
