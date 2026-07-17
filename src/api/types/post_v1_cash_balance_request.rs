pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CashBalanceRequest {
    #[serde(rename = "cashAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_account_code: Option<String>,
    #[serde(rename = "asOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
}

impl PostV1CashBalanceRequest {
    pub fn builder() -> PostV1CashBalanceRequestBuilder {
        <PostV1CashBalanceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CashBalanceRequestBuilder {
    cash_account_code: Option<String>,
    as_of: Option<String>,
}

impl PostV1CashBalanceRequestBuilder {
    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CashBalanceRequest`].
    pub fn build(self) -> Result<PostV1CashBalanceRequest, BuildError> {
        Ok(PostV1CashBalanceRequest {
            cash_account_code: self.cash_account_code,
            as_of: self.as_of,
        })
    }
}
