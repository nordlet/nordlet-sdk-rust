pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StatementRowsSetLedgerRequest {
    #[serde(default)]
    pub scheme: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "rowCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_code: Option<String>,
}

impl StatementRowsSetLedgerRequest {
    pub fn builder() -> StatementRowsSetLedgerRequestBuilder {
        <StatementRowsSetLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatementRowsSetLedgerRequestBuilder {
    scheme: Option<String>,
    account_code: Option<String>,
    row_code: Option<String>,
}

impl StatementRowsSetLedgerRequestBuilder {
    pub fn scheme(mut self, value: impl Into<String>) -> Self {
        self.scheme = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn row_code(mut self, value: impl Into<String>) -> Self {
        self.row_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatementRowsSetLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](StatementRowsSetLedgerRequestBuilder::scheme)
    /// - [`account_code`](StatementRowsSetLedgerRequestBuilder::account_code)
    pub fn build(self) -> Result<StatementRowsSetLedgerRequest, BuildError> {
        Ok(StatementRowsSetLedgerRequest {
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            row_code: self.row_code,
        })
    }
}
