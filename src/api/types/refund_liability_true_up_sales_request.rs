pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundLiabilityTrueUpSalesRequest {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "estimatedTotal")]
    #[serde(default)]
    pub estimated_total: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl RefundLiabilityTrueUpSalesRequest {
    pub fn builder() -> RefundLiabilityTrueUpSalesRequestBuilder {
        <RefundLiabilityTrueUpSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLiabilityTrueUpSalesRequestBuilder {
    invoice_id: Option<String>,
    estimated_total: Option<String>,
    date: Option<NaiveDate>,
}

impl RefundLiabilityTrueUpSalesRequestBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn estimated_total(mut self, value: impl Into<String>) -> Self {
        self.estimated_total = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLiabilityTrueUpSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](RefundLiabilityTrueUpSalesRequestBuilder::invoice_id)
    /// - [`estimated_total`](RefundLiabilityTrueUpSalesRequestBuilder::estimated_total)
    pub fn build(self) -> Result<RefundLiabilityTrueUpSalesRequest, BuildError> {
        Ok(RefundLiabilityTrueUpSalesRequest {
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
