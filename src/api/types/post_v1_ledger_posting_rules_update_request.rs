pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesUpdateRequest {
    #[serde(default)]
    pub rules: Vec<PostV1LedgerPostingRulesUpdateRequestRulesItem>,
}

impl PostV1LedgerPostingRulesUpdateRequest {
    pub fn builder() -> PostV1LedgerPostingRulesUpdateRequestBuilder {
        <PostV1LedgerPostingRulesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesUpdateRequestBuilder {
    rules: Option<Vec<PostV1LedgerPostingRulesUpdateRequestRulesItem>>,
}

impl PostV1LedgerPostingRulesUpdateRequestBuilder {
    pub fn rules(mut self, value: Vec<PostV1LedgerPostingRulesUpdateRequestRulesItem>) -> Self {
        self.rules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](PostV1LedgerPostingRulesUpdateRequestBuilder::rules)
    pub fn build(self) -> Result<PostV1LedgerPostingRulesUpdateRequest, BuildError> {
        Ok(PostV1LedgerPostingRulesUpdateRequest {
            rules: self
                .rules
                .ok_or_else(|| BuildError::missing_field("rules"))?,
        })
    }
}
