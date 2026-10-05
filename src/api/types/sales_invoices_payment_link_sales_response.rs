pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesPaymentLinkSalesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<InvoicesPaymentLinkSalesResponseSource>,
}

impl InvoicesPaymentLinkSalesResponse {
    pub fn builder() -> InvoicesPaymentLinkSalesResponseBuilder {
        <InvoicesPaymentLinkSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesPaymentLinkSalesResponseBuilder {
    url: Option<String>,
    source: Option<InvoicesPaymentLinkSalesResponseSource>,
}

impl InvoicesPaymentLinkSalesResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn source(mut self, value: InvoicesPaymentLinkSalesResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesPaymentLinkSalesResponse`].
    pub fn build(self) -> Result<InvoicesPaymentLinkSalesResponse, BuildError> {
        Ok(InvoicesPaymentLinkSalesResponse {
            url: self.url,
            source: self.source,
        })
    }
}
