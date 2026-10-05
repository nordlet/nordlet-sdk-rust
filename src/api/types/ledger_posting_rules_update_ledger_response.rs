pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostingRulesUpdateLedgerResponse {
    #[serde(default)]
    pub rows: Vec<PostingRulesUpdateLedgerResponseRowsItem>,
}

impl PostingRulesUpdateLedgerResponse {
    pub fn builder() -> PostingRulesUpdateLedgerResponseBuilder {
        <PostingRulesUpdateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostingRulesUpdateLedgerResponseBuilder {
    rows: Option<Vec<PostingRulesUpdateLedgerResponseRowsItem>>,
}

impl PostingRulesUpdateLedgerResponseBuilder {
    pub fn rows(mut self, value: Vec<PostingRulesUpdateLedgerResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostingRulesUpdateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostingRulesUpdateLedgerResponseBuilder::rows)
    pub fn build(self) -> Result<PostingRulesUpdateLedgerResponse, BuildError> {
        Ok(PostingRulesUpdateLedgerResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
