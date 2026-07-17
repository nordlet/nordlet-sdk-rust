pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesUpdateRequestRulesItem {
    pub key: PostV1LedgerPostingRulesUpdateRequestRulesItemKey,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}

impl PostV1LedgerPostingRulesUpdateRequestRulesItem {
    pub fn builder() -> PostV1LedgerPostingRulesUpdateRequestRulesItemBuilder {
        <PostV1LedgerPostingRulesUpdateRequestRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesUpdateRequestRulesItemBuilder {
    key: Option<PostV1LedgerPostingRulesUpdateRequestRulesItemKey>,
    account_code: Option<String>,
}

impl PostV1LedgerPostingRulesUpdateRequestRulesItemBuilder {
    pub fn key(mut self, value: PostV1LedgerPostingRulesUpdateRequestRulesItemKey) -> Self {
        self.key = Some(value);
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesUpdateRequestRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1LedgerPostingRulesUpdateRequestRulesItemBuilder::key)
    pub fn build(self) -> Result<PostV1LedgerPostingRulesUpdateRequestRulesItem, BuildError> {
        Ok(PostV1LedgerPostingRulesUpdateRequestRulesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            account_code: self.account_code,
        })
    }
}
