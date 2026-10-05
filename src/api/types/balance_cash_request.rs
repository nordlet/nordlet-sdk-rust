pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BalanceCashRequest {
    #[serde(rename = "cashAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_account_code: Option<String>,
    #[serde(rename = "asOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<NaiveDate>,
}

impl BalanceCashRequest {
    pub fn builder() -> BalanceCashRequestBuilder {
        <BalanceCashRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BalanceCashRequestBuilder {
    cash_account_code: Option<String>,
    as_of: Option<NaiveDate>,
}

impl BalanceCashRequestBuilder {
    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn as_of(mut self, value: NaiveDate) -> Self {
        self.as_of = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BalanceCashRequest`].
    pub fn build(self) -> Result<BalanceCashRequest, BuildError> {
        Ok(BalanceCashRequest {
            cash_account_code: self.cash_account_code,
            as_of: self.as_of,
        })
    }
}
