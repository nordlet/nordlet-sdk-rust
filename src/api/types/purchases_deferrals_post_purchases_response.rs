pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeferralsPostPurchasesResponse {
    #[serde(default)]
    pub posted: i64,
    #[serde(default)]
    pub total: String,
    #[serde(rename = "journalTransactionIds")]
    #[serde(default)]
    pub journal_transaction_ids: Vec<String>,
}

impl DeferralsPostPurchasesResponse {
    pub fn builder() -> DeferralsPostPurchasesResponseBuilder {
        <DeferralsPostPurchasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeferralsPostPurchasesResponseBuilder {
    posted: Option<i64>,
    total: Option<String>,
    journal_transaction_ids: Option<Vec<String>>,
}

impl DeferralsPostPurchasesResponseBuilder {
    pub fn posted(mut self, value: i64) -> Self {
        self.posted = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn journal_transaction_ids(mut self, value: Vec<String>) -> Self {
        self.journal_transaction_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeferralsPostPurchasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`posted`](DeferralsPostPurchasesResponseBuilder::posted)
    /// - [`total`](DeferralsPostPurchasesResponseBuilder::total)
    /// - [`journal_transaction_ids`](DeferralsPostPurchasesResponseBuilder::journal_transaction_ids)
    pub fn build(self) -> Result<DeferralsPostPurchasesResponse, BuildError> {
        Ok(DeferralsPostPurchasesResponse {
            posted: self
                .posted
                .ok_or_else(|| BuildError::missing_field("posted"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            journal_transaction_ids: self
                .journal_transaction_ids
                .ok_or_else(|| BuildError::missing_field("journal_transaction_ids"))?,
        })
    }
}
