pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsSuggestMatchesBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl TransactionsSuggestMatchesBankRequest {
    pub fn builder() -> TransactionsSuggestMatchesBankRequestBuilder {
        <TransactionsSuggestMatchesBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsSuggestMatchesBankRequestBuilder {
    transaction_id: Option<String>,
    limit: Option<i64>,
}

impl TransactionsSuggestMatchesBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsSuggestMatchesBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsSuggestMatchesBankRequestBuilder::transaction_id)
    pub fn build(self) -> Result<TransactionsSuggestMatchesBankRequest, BuildError> {
        Ok(TransactionsSuggestMatchesBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            limit: self.limit,
        })
    }
}
