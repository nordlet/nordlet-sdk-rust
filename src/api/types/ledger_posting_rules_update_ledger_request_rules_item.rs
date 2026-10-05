pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostingRulesUpdateLedgerRequestRulesItem {
    pub key: PostingRulesUpdateLedgerRequestRulesItemKey,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}

impl PostingRulesUpdateLedgerRequestRulesItem {
    pub fn builder() -> PostingRulesUpdateLedgerRequestRulesItemBuilder {
        <PostingRulesUpdateLedgerRequestRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostingRulesUpdateLedgerRequestRulesItemBuilder {
    key: Option<PostingRulesUpdateLedgerRequestRulesItemKey>,
    account_code: Option<String>,
}

impl PostingRulesUpdateLedgerRequestRulesItemBuilder {
    pub fn key(mut self, value: PostingRulesUpdateLedgerRequestRulesItemKey) -> Self {
        self.key = Some(value);
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostingRulesUpdateLedgerRequestRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostingRulesUpdateLedgerRequestRulesItemBuilder::key)
    pub fn build(self) -> Result<PostingRulesUpdateLedgerRequestRulesItem, BuildError> {
        Ok(PostingRulesUpdateLedgerRequestRulesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            account_code: self.account_code,
        })
    }
}
