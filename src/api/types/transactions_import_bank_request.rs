pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsImportBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub transactions: Vec<TransactionsImportBankRequestTransactionsItem>,
}

impl TransactionsImportBankRequest {
    pub fn builder() -> TransactionsImportBankRequestBuilder {
        <TransactionsImportBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsImportBankRequestBuilder {
    bank_account_id: Option<String>,
    transactions: Option<Vec<TransactionsImportBankRequestTransactionsItem>>,
}

impl TransactionsImportBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn transactions(
        mut self,
        value: Vec<TransactionsImportBankRequestTransactionsItem>,
    ) -> Self {
        self.transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsImportBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](TransactionsImportBankRequestBuilder::bank_account_id)
    /// - [`transactions`](TransactionsImportBankRequestBuilder::transactions)
    pub fn build(self) -> Result<TransactionsImportBankRequest, BuildError> {
        Ok(TransactionsImportBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            transactions: self
                .transactions
                .ok_or_else(|| BuildError::missing_field("transactions"))?,
        })
    }
}
