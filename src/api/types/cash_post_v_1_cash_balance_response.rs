pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CashBalanceResponse {
    #[serde(rename = "cashAccountCode")]
    #[serde(default)]
    pub cash_account_code: String,
    #[serde(default)]
    pub balance: String,
}

impl PostV1CashBalanceResponse {
    pub fn builder() -> PostV1CashBalanceResponseBuilder {
        <PostV1CashBalanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashBalanceResponseBuilder {
    cash_account_code: Option<String>,
    balance: Option<String>,
}

impl PostV1CashBalanceResponseBuilder {
    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashBalanceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cash_account_code`](PostV1CashBalanceResponseBuilder::cash_account_code)
    /// - [`balance`](PostV1CashBalanceResponseBuilder::balance)
    pub fn build(self) -> Result<PostV1CashBalanceResponse, BuildError> {
        Ok(PostV1CashBalanceResponse {
            cash_account_code: self
                .cash_account_code
                .ok_or_else(|| BuildError::missing_field("cash_account_code"))?,
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
        })
    }
}
