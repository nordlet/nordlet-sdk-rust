pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsImportBankResponseDuplicatesItem {
    #[serde(default)]
    pub index: i64,
    #[serde(rename = "existingTransactionId")]
    #[serde(default)]
    pub existing_transaction_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub amount: String,
}

impl TransactionsImportBankResponseDuplicatesItem {
    pub fn builder() -> TransactionsImportBankResponseDuplicatesItemBuilder {
        <TransactionsImportBankResponseDuplicatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsImportBankResponseDuplicatesItemBuilder {
    index: Option<i64>,
    existing_transaction_id: Option<String>,
    date: Option<NaiveDate>,
    amount: Option<String>,
}

impl TransactionsImportBankResponseDuplicatesItemBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn existing_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.existing_transaction_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsImportBankResponseDuplicatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](TransactionsImportBankResponseDuplicatesItemBuilder::index)
    /// - [`existing_transaction_id`](TransactionsImportBankResponseDuplicatesItemBuilder::existing_transaction_id)
    /// - [`date`](TransactionsImportBankResponseDuplicatesItemBuilder::date)
    /// - [`amount`](TransactionsImportBankResponseDuplicatesItemBuilder::amount)
    pub fn build(self) -> Result<TransactionsImportBankResponseDuplicatesItem, BuildError> {
        Ok(TransactionsImportBankResponseDuplicatesItem {
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            existing_transaction_id: self
                .existing_transaction_id
                .ok_or_else(|| BuildError::missing_field("existing_transaction_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
