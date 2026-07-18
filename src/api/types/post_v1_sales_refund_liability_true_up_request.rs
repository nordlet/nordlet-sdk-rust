pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRefundLiabilityTrueUpRequest {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "estimatedTotal")]
    #[serde(default)]
    pub estimated_total: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl PostV1SalesRefundLiabilityTrueUpRequest {
    pub fn builder() -> PostV1SalesRefundLiabilityTrueUpRequestBuilder {
        <PostV1SalesRefundLiabilityTrueUpRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRefundLiabilityTrueUpRequestBuilder {
    invoice_id: Option<String>,
    estimated_total: Option<String>,
    date: Option<String>,
}

impl PostV1SalesRefundLiabilityTrueUpRequestBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn estimated_total(mut self, value: impl Into<String>) -> Self {
        self.estimated_total = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRefundLiabilityTrueUpRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1SalesRefundLiabilityTrueUpRequestBuilder::invoice_id)
    /// - [`estimated_total`](PostV1SalesRefundLiabilityTrueUpRequestBuilder::estimated_total)
    pub fn build(self) -> Result<PostV1SalesRefundLiabilityTrueUpRequest, BuildError> {
        Ok(PostV1SalesRefundLiabilityTrueUpRequest {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            estimated_total: self
                .estimated_total
                .ok_or_else(|| BuildError::missing_field("estimated_total"))?,
            date: self.date,
        })
    }
}
