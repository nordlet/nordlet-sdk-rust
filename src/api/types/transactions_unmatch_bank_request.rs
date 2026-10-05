pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsUnmatchBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
}

impl TransactionsUnmatchBankRequest {
    pub fn builder() -> TransactionsUnmatchBankRequestBuilder {
        <TransactionsUnmatchBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsUnmatchBankRequestBuilder {
    transaction_id: Option<String>,
    date: Option<NaiveDate>,
}

impl TransactionsUnmatchBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsUnmatchBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsUnmatchBankRequestBuilder::transaction_id)
    pub fn build(self) -> Result<TransactionsUnmatchBankRequest, BuildError> {
        Ok(TransactionsUnmatchBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            date: self.date,
        })
    }
}
