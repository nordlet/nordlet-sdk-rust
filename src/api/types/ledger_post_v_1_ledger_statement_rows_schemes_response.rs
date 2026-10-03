pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerStatementRowsSchemesResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LedgerStatementRowsSchemesResponseRowsItem>,
}

impl PostV1LedgerStatementRowsSchemesResponse {
    pub fn builder() -> PostV1LedgerStatementRowsSchemesResponseBuilder {
        <PostV1LedgerStatementRowsSchemesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerStatementRowsSchemesResponseBuilder {
    rows: Option<Vec<PostV1LedgerStatementRowsSchemesResponseRowsItem>>,
}

impl PostV1LedgerStatementRowsSchemesResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LedgerStatementRowsSchemesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerStatementRowsSchemesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LedgerStatementRowsSchemesResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LedgerStatementRowsSchemesResponse, BuildError> {
        Ok(PostV1LedgerStatementRowsSchemesResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
