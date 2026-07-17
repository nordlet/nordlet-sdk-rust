pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesUpdateResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LedgerPostingRulesUpdateResponseRowsItem>,
}

impl PostV1LedgerPostingRulesUpdateResponse {
    pub fn builder() -> PostV1LedgerPostingRulesUpdateResponseBuilder {
        <PostV1LedgerPostingRulesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesUpdateResponseBuilder {
    rows: Option<Vec<PostV1LedgerPostingRulesUpdateResponseRowsItem>>,
}

impl PostV1LedgerPostingRulesUpdateResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LedgerPostingRulesUpdateResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LedgerPostingRulesUpdateResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LedgerPostingRulesUpdateResponse, BuildError> {
        Ok(PostV1LedgerPostingRulesUpdateResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
