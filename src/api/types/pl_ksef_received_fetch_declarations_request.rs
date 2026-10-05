pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlKsefReceivedFetchDeclarationsRequest {
    #[serde(rename = "ksefNumber")]
    #[serde(default)]
    pub ksef_number: String,
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_invoice_id: Option<String>,
}

impl PlKsefReceivedFetchDeclarationsRequest {
    pub fn builder() -> PlKsefReceivedFetchDeclarationsRequestBuilder {
        <PlKsefReceivedFetchDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlKsefReceivedFetchDeclarationsRequestBuilder {
    ksef_number: Option<String>,
    purchase_invoice_id: Option<String>,
}

impl PlKsefReceivedFetchDeclarationsRequestBuilder {
    pub fn ksef_number(mut self, value: impl Into<String>) -> Self {
        self.ksef_number = Some(value.into());
        self
    }

    pub fn purchase_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlKsefReceivedFetchDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ksef_number`](PlKsefReceivedFetchDeclarationsRequestBuilder::ksef_number)
    pub fn build(self) -> Result<PlKsefReceivedFetchDeclarationsRequest, BuildError> {
        Ok(PlKsefReceivedFetchDeclarationsRequest {
            ksef_number: self
                .ksef_number
                .ok_or_else(|| BuildError::missing_field("ksef_number"))?,
            purchase_invoice_id: self.purchase_invoice_id,
        })
    }
}
