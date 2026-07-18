pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRefundLiabilityTrueUpResponse {
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

impl PostV1SalesRefundLiabilityTrueUpResponse {
    pub fn builder() -> PostV1SalesRefundLiabilityTrueUpResponseBuilder {
        <PostV1SalesRefundLiabilityTrueUpResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRefundLiabilityTrueUpResponseBuilder {
    invoice_id: Option<String>,
    estimated: Option<String>,
    consumed: Option<String>,
    remaining: Option<String>,
    delta: Option<String>,
    journal_transaction_id: Option<String>,
}

impl PostV1SalesRefundLiabilityTrueUpResponseBuilder {
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

    /// Consumes the builder and constructs a [`PostV1SalesRefundLiabilityTrueUpResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1SalesRefundLiabilityTrueUpResponseBuilder::invoice_id)
    /// - [`estimated`](PostV1SalesRefundLiabilityTrueUpResponseBuilder::estimated)
    /// - [`consumed`](PostV1SalesRefundLiabilityTrueUpResponseBuilder::consumed)
    /// - [`remaining`](PostV1SalesRefundLiabilityTrueUpResponseBuilder::remaining)
    /// - [`delta`](PostV1SalesRefundLiabilityTrueUpResponseBuilder::delta)
    /// - [`journal_transaction_id`](PostV1SalesRefundLiabilityTrueUpResponseBuilder::journal_transaction_id)
    pub fn build(self) -> Result<PostV1SalesRefundLiabilityTrueUpResponse, BuildError> {
        Ok(PostV1SalesRefundLiabilityTrueUpResponse {
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
