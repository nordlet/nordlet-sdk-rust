pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostingRulesListLedgerResponse {
    #[serde(default)]
    pub rows: Vec<PostingRulesListLedgerResponseRowsItem>,
}

impl PostingRulesListLedgerResponse {
    pub fn builder() -> PostingRulesListLedgerResponseBuilder {
        <PostingRulesListLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostingRulesListLedgerResponseBuilder {
    rows: Option<Vec<PostingRulesListLedgerResponseRowsItem>>,
}

impl PostingRulesListLedgerResponseBuilder {
    pub fn rows(mut self, value: Vec<PostingRulesListLedgerResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostingRulesListLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostingRulesListLedgerResponseBuilder::rows)
    pub fn build(self) -> Result<PostingRulesListLedgerResponse, BuildError> {
        Ok(PostingRulesListLedgerResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
