pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsListResponse {
    #[serde(default)]
    pub scheme: PostV1LedgerStatementRowsListResponseScheme,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub accounts: Vec<PostV1LedgerStatementRowsListResponseAccountsItem>,
    #[serde(default)]
    pub rows: Vec<PostV1LedgerStatementRowsListResponseRowsItem>,
    #[serde(default)]
    pub unmapped: Vec<String>,
}

impl PostV1LedgerStatementRowsListResponse {
    pub fn builder() -> PostV1LedgerStatementRowsListResponseBuilder {
        <PostV1LedgerStatementRowsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsListResponseBuilder {
    scheme: Option<PostV1LedgerStatementRowsListResponseScheme>,
    from_date: Option<String>,
    to_date: Option<String>,
    accounts: Option<Vec<PostV1LedgerStatementRowsListResponseAccountsItem>>,
    rows: Option<Vec<PostV1LedgerStatementRowsListResponseRowsItem>>,
    unmapped: Option<Vec<String>>,
}

impl PostV1LedgerStatementRowsListResponseBuilder {
    pub fn scheme(mut self, value: PostV1LedgerStatementRowsListResponseScheme) -> Self {
        self.scheme = Some(value);
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

    pub fn accounts(
        mut self,
        value: Vec<PostV1LedgerStatementRowsListResponseAccountsItem>,
    ) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<PostV1LedgerStatementRowsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn unmapped(mut self, value: Vec<String>) -> Self {
        self.unmapped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheme`](PostV1LedgerStatementRowsListResponseBuilder::scheme)
    /// - [`from_date`](PostV1LedgerStatementRowsListResponseBuilder::from_date)
    /// - [`to_date`](PostV1LedgerStatementRowsListResponseBuilder::to_date)
    /// - [`accounts`](PostV1LedgerStatementRowsListResponseBuilder::accounts)
    /// - [`rows`](PostV1LedgerStatementRowsListResponseBuilder::rows)
    /// - [`unmapped`](PostV1LedgerStatementRowsListResponseBuilder::unmapped)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsListResponse, BuildError> {
        Ok(PostV1LedgerStatementRowsListResponse {
            scheme: self
                .scheme
                .ok_or_else(|| BuildError::missing_field("scheme"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            unmapped: self
                .unmapped
                .ok_or_else(|| BuildError::missing_field("unmapped"))?,
        })
    }
}
