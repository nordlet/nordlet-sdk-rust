pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsMatchManyBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(default)]
    pub allocations: Vec<TransactionsMatchManyBankRequestAllocationsItem>,
}

impl TransactionsMatchManyBankRequest {
    pub fn builder() -> TransactionsMatchManyBankRequestBuilder {
        <TransactionsMatchManyBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsMatchManyBankRequestBuilder {
    transaction_id: Option<String>,
    allocations: Option<Vec<TransactionsMatchManyBankRequestAllocationsItem>>,
}

impl TransactionsMatchManyBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn allocations(
        mut self,
        value: Vec<TransactionsMatchManyBankRequestAllocationsItem>,
    ) -> Self {
        self.allocations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsMatchManyBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsMatchManyBankRequestBuilder::transaction_id)
    /// - [`allocations`](TransactionsMatchManyBankRequestBuilder::allocations)
    pub fn build(self) -> Result<TransactionsMatchManyBankRequest, BuildError> {
        Ok(TransactionsMatchManyBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            allocations: self
                .allocations
                .ok_or_else(|| BuildError::missing_field("allocations"))?,
        })
    }
}
