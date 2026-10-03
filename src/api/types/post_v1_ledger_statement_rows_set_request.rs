pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsSetRequest {
    #[serde(default)]
    pub scheme: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "rowCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_code: Option<String>,
}

impl PostV1LedgerStatementRowsSetRequest {
    pub fn builder() -> PostV1LedgerStatementRowsSetRequestBuilder {
        <PostV1LedgerStatementRowsSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsSetRequestBuilder {
    scheme: Option<String>,
    account_code: Option<String>,
    row_code: Option<String>,
}

impl PostV1LedgerStatementRowsSetRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](PostV1LedgerStatementRowsSetRequestBuilder::scheme)
    /// - [`account_code`](PostV1LedgerStatementRowsSetRequestBuilder::account_code)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsSetRequest, BuildError> {
        Ok(PostV1LedgerStatementRowsSetRequest {
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
