pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseOpeningBalances {
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub entries: i64,
    #[serde(rename = "debitTotal")]
    #[serde(default)]
    pub debit_total: String,
    #[serde(rename = "creditTotal")]
    #[serde(default)]
    pub credit_total: String,
    #[serde(rename = "balancingAmount")]
    #[serde(default)]
    pub balancing_amount: String,
}

impl PostV1MigrationBooksImportResponseOpeningBalances {
    pub fn builder() -> PostV1MigrationBooksImportResponseOpeningBalancesBuilder {
        <PostV1MigrationBooksImportResponseOpeningBalancesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseOpeningBalancesBuilder {
    journal_transaction_id: Option<String>,
    date: Option<String>,
    entries: Option<i64>,
    debit_total: Option<String>,
    credit_total: Option<String>,
    balancing_amount: Option<String>,
}

impl PostV1MigrationBooksImportResponseOpeningBalancesBuilder {
    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn entries(mut self, value: i64) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn debit_total(mut self, value: impl Into<String>) -> Self {
        self.debit_total = Some(value.into());
        self
    }

    pub fn credit_total(mut self, value: impl Into<String>) -> Self {
        self.credit_total = Some(value.into());
        self
    }

    pub fn balancing_amount(mut self, value: impl Into<String>) -> Self {
        self.balancing_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseOpeningBalances`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1MigrationBooksImportResponseOpeningBalancesBuilder::date)
    /// - [`entries`](PostV1MigrationBooksImportResponseOpeningBalancesBuilder::entries)
    /// - [`debit_total`](PostV1MigrationBooksImportResponseOpeningBalancesBuilder::debit_total)
    /// - [`credit_total`](PostV1MigrationBooksImportResponseOpeningBalancesBuilder::credit_total)
    /// - [`balancing_amount`](PostV1MigrationBooksImportResponseOpeningBalancesBuilder::balancing_amount)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseOpeningBalances, BuildError> {
        Ok(PostV1MigrationBooksImportResponseOpeningBalances {
            journal_transaction_id: self.journal_transaction_id,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
            debit_total: self
                .debit_total
                .ok_or_else(|| BuildError::missing_field("debit_total"))?,
            credit_total: self
                .credit_total
                .ok_or_else(|| BuildError::missing_field("credit_total"))?,
            balancing_amount: self
                .balancing_amount
                .ok_or_else(|| BuildError::missing_field("balancing_amount"))?,
        })
    }
}
