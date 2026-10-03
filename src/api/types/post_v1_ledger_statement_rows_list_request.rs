pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsListRequest {
    #[serde(default)]
    pub scheme: String,
    #[serde(rename = "fromDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    #[serde(rename = "toDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
}

impl PostV1LedgerStatementRowsListRequest {
    pub fn builder() -> PostV1LedgerStatementRowsListRequestBuilder {
        <PostV1LedgerStatementRowsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsListRequestBuilder {
    scheme: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
}

impl PostV1LedgerStatementRowsListRequestBuilder {
    pub fn scheme(mut self, value: impl Into<String>) -> Self {
        self.scheme = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](PostV1LedgerStatementRowsListRequestBuilder::scheme)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsListRequest, BuildError> {
        Ok(PostV1LedgerStatementRowsListRequest {
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            from_date: self.from_date,
            to_date: self.to_date,
        })
    }
}
