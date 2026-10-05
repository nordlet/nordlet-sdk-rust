pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundLiabilityTrueUpSalesResponse {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(default)]
    pub estimated: String,
    #[serde(default)]
    pub consumed: String,
    #[serde(default)]
    pub remaining: String,
    #[serde(default)]
    pub delta: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(default)]
    pub journal_transaction_id: String,
}

impl RefundLiabilityTrueUpSalesResponse {
    pub fn builder() -> RefundLiabilityTrueUpSalesResponseBuilder {
        <RefundLiabilityTrueUpSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLiabilityTrueUpSalesResponseBuilder {
    invoice_id: Option<String>,
    estimated: Option<String>,
    consumed: Option<String>,
    remaining: Option<String>,
    delta: Option<String>,
    journal_transaction_id: Option<String>,
}

impl RefundLiabilityTrueUpSalesResponseBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn estimated(mut self, value: impl Into<String>) -> Self {
        self.estimated = Some(value.into());
        self
    }

    pub fn consumed(mut self, value: impl Into<String>) -> Self {
        self.consumed = Some(value.into());
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn delta(mut self, value: impl Into<String>) -> Self {
        self.delta = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefundLiabilityTrueUpSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](RefundLiabilityTrueUpSalesResponseBuilder::invoice_id)
    /// - [`estimated`](RefundLiabilityTrueUpSalesResponseBuilder::estimated)
    /// - [`consumed`](RefundLiabilityTrueUpSalesResponseBuilder::consumed)
    /// - [`remaining`](RefundLiabilityTrueUpSalesResponseBuilder::remaining)
    /// - [`delta`](RefundLiabilityTrueUpSalesResponseBuilder::delta)
    /// - [`journal_transaction_id`](RefundLiabilityTrueUpSalesResponseBuilder::journal_transaction_id)
    pub fn build(self) -> Result<RefundLiabilityTrueUpSalesResponse, BuildError> {
        Ok(RefundLiabilityTrueUpSalesResponse {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            estimated: self
                .estimated
                .ok_or_else(|| BuildError::missing_field("estimated"))?,
            consumed: self
                .consumed
                .ok_or_else(|| BuildError::missing_field("consumed"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            delta: self
                .delta
                .ok_or_else(|| BuildError::missing_field("delta"))?,
            journal_transaction_id: self
                .journal_transaction_id
                .ok_or_else(|| BuildError::missing_field("journal_transaction_id"))?,
        })
    }
}
