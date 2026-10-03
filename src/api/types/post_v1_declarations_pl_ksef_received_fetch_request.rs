pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlKsefReceivedFetchRequest {
    #[serde(rename = "ksefNumber")]
    #[serde(default)]
    pub ksef_number: String,
    #[serde(rename = "purchaseInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_invoice_id: Option<String>,
}

impl PostV1DeclarationsPlKsefReceivedFetchRequest {
    pub fn builder() -> PostV1DeclarationsPlKsefReceivedFetchRequestBuilder {
        <PostV1DeclarationsPlKsefReceivedFetchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlKsefReceivedFetchRequestBuilder {
    ksef_number: Option<String>,
    purchase_invoice_id: Option<String>,
}

impl PostV1DeclarationsPlKsefReceivedFetchRequestBuilder {
    pub fn ksef_number(mut self, value: impl Into<String>) -> Self {
        self.ksef_number = Some(value.into());
        self
    }

    pub fn purchase_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.purchase_invoice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlKsefReceivedFetchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ksef_number`](PostV1DeclarationsPlKsefReceivedFetchRequestBuilder::ksef_number)
    pub fn build(self) -> Result<PostV1DeclarationsPlKsefReceivedFetchRequest, BuildError> {
        Ok(PostV1DeclarationsPlKsefReceivedFetchRequest {
            ksef_number: self
                .ksef_number
                .ok_or_else(|| BuildError::missing_field("ksef_number"))?,
            purchase_invoice_id: self.purchase_invoice_id,
        })
    }
}
