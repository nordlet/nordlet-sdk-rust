pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BalanceCashResponse {
    #[serde(rename = "cashAccountCode")]
    #[serde(default)]
    pub cash_account_code: String,
    #[serde(default)]
    pub balance: String,
}

impl BalanceCashResponse {
    pub fn builder() -> BalanceCashResponseBuilder {
        <BalanceCashResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BalanceCashResponseBuilder {
    cash_account_code: Option<String>,
    balance: Option<String>,
}

impl BalanceCashResponseBuilder {
    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BalanceCashResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cash_account_code`](BalanceCashResponseBuilder::cash_account_code)
    /// - [`balance`](BalanceCashResponseBuilder::balance)
    pub fn build(self) -> Result<BalanceCashResponse, BuildError> {
        Ok(BalanceCashResponse {
            cash_account_code: self
                .cash_account_code
                .ok_or_else(|| BuildError::missing_field("cash_account_code"))?,
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
        })
    }
}
