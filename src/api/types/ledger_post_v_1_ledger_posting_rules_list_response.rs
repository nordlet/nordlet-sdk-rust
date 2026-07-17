pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LedgerPostingRulesListResponseRowsItem>,
}

impl PostV1LedgerPostingRulesListResponse {
    pub fn builder() -> PostV1LedgerPostingRulesListResponseBuilder {
        <PostV1LedgerPostingRulesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesListResponseBuilder {
    rows: Option<Vec<PostV1LedgerPostingRulesListResponseRowsItem>>,
}

impl PostV1LedgerPostingRulesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LedgerPostingRulesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LedgerPostingRulesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LedgerPostingRulesListResponse, BuildError> {
        Ok(PostV1LedgerPostingRulesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
