pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesInvoicesMatchRequest {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "priceTolerancePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_tolerance_percent: Option<String>,
}

impl PostV1PurchasesInvoicesMatchRequest {
    pub fn builder() -> PostV1PurchasesInvoicesMatchRequestBuilder {
        <PostV1PurchasesInvoicesMatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesInvoicesMatchRequestBuilder {
    invoice_id: Option<String>,
    price_tolerance_percent: Option<String>,
}

impl PostV1PurchasesInvoicesMatchRequestBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn price_tolerance_percent(mut self, value: impl Into<String>) -> Self {
        self.price_tolerance_percent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesInvoicesMatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1PurchasesInvoicesMatchRequestBuilder::invoice_id)
    pub fn build(self) -> Result<PostV1PurchasesInvoicesMatchRequest, BuildError> {
        Ok(PostV1PurchasesInvoicesMatchRequest {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            price_tolerance_percent: self.price_tolerance_percent,
        })
    }
}
