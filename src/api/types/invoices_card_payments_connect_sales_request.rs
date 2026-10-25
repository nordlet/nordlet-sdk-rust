pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesCardPaymentsConnectSalesRequest {
    #[serde(rename = "returnUrl")]
    #[serde(default)]
    pub return_url: String,
    #[serde(rename = "refreshUrl")]
    #[serde(default)]
    pub refresh_url: String,
}

impl InvoicesCardPaymentsConnectSalesRequest {
    pub fn builder() -> InvoicesCardPaymentsConnectSalesRequestBuilder {
        <InvoicesCardPaymentsConnectSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesCardPaymentsConnectSalesRequestBuilder {
    return_url: Option<String>,
    refresh_url: Option<String>,
}

impl InvoicesCardPaymentsConnectSalesRequestBuilder {
    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn refresh_url(mut self, value: impl Into<String>) -> Self {
        self.refresh_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesCardPaymentsConnectSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`return_url`](InvoicesCardPaymentsConnectSalesRequestBuilder::return_url)
    /// - [`refresh_url`](InvoicesCardPaymentsConnectSalesRequestBuilder::refresh_url)
    pub fn build(self) -> Result<InvoicesCardPaymentsConnectSalesRequest, BuildError> {
        Ok(InvoicesCardPaymentsConnectSalesRequest {
            return_url: self
                .return_url
                .ok_or_else(|| BuildError::missing_field("return_url"))?,
            refresh_url: self
                .refresh_url
                .ok_or_else(|| BuildError::missing_field("refresh_url"))?,
        })
    }
}
