pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsImportRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub transactions: Vec<PostV1BankTransactionsImportRequestTransactionsItem>,
}

impl PostV1BankTransactionsImportRequest {
    pub fn builder() -> PostV1BankTransactionsImportRequestBuilder {
        <PostV1BankTransactionsImportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsImportRequestBuilder {
    bank_account_id: Option<String>,
    transactions: Option<Vec<PostV1BankTransactionsImportRequestTransactionsItem>>,
}

impl PostV1BankTransactionsImportRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn transactions(
        mut self,
        value: Vec<PostV1BankTransactionsImportRequestTransactionsItem>,
    ) -> Self {
        self.transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsImportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](PostV1BankTransactionsImportRequestBuilder::bank_account_id)
    /// - [`transactions`](PostV1BankTransactionsImportRequestBuilder::transactions)
    pub fn build(self) -> Result<PostV1BankTransactionsImportRequest, BuildError> {
        Ok(PostV1BankTransactionsImportRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            transactions: self
                .transactions
                .ok_or_else(|| BuildError::missing_field("transactions"))?,
        })
    }
}
