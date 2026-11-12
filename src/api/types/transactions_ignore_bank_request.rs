pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsIgnoreBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(default)]
    pub reason: String,
}

impl TransactionsIgnoreBankRequest {
    pub fn builder() -> TransactionsIgnoreBankRequestBuilder {
        <TransactionsIgnoreBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsIgnoreBankRequestBuilder {
    transaction_id: Option<String>,
    reason: Option<String>,
}

impl TransactionsIgnoreBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsIgnoreBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsIgnoreBankRequestBuilder::transaction_id)
    /// - [`reason`](TransactionsIgnoreBankRequestBuilder::reason)
    pub fn build(self) -> Result<TransactionsIgnoreBankRequest, BuildError> {
        Ok(TransactionsIgnoreBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
