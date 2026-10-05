pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostingRulesUpdateLedgerRequest {
    #[serde(default)]
    pub rules: Vec<PostingRulesUpdateLedgerRequestRulesItem>,
}

impl PostingRulesUpdateLedgerRequest {
    pub fn builder() -> PostingRulesUpdateLedgerRequestBuilder {
        <PostingRulesUpdateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostingRulesUpdateLedgerRequestBuilder {
    rules: Option<Vec<PostingRulesUpdateLedgerRequestRulesItem>>,
}

impl PostingRulesUpdateLedgerRequestBuilder {
    pub fn rules(mut self, value: Vec<PostingRulesUpdateLedgerRequestRulesItem>) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostingRulesUpdateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](PostingRulesUpdateLedgerRequestBuilder::rules)
    pub fn build(self) -> Result<PostingRulesUpdateLedgerRequest, BuildError> {
        Ok(PostingRulesUpdateLedgerRequest {
            rules: self
                .rules
                .ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}
