pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsUnignoreBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

impl TransactionsUnignoreBankRequest {
    pub fn builder() -> TransactionsUnignoreBankRequestBuilder {
        <TransactionsUnignoreBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsUnignoreBankRequestBuilder {
    transaction_id: Option<String>,
}

impl TransactionsUnignoreBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsUnignoreBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsUnignoreBankRequestBuilder::transaction_id)
    pub fn build(self) -> Result<TransactionsUnignoreBankRequest, BuildError> {
        Ok(TransactionsUnignoreBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
        })
    }
}
