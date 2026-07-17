pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesUpdateResponseRowsItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "defaultCode")]
    #[serde(default)]
    pub default_code: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(default)]
    pub overridden: bool,
}

impl PostV1LedgerPostingRulesUpdateResponseRowsItem {
    pub fn builder() -> PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder {
        <PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder {
    key: Option<String>,
    description: Option<String>,
    default_code: Option<String>,
    account_code: Option<String>,
    overridden: Option<bool>,
}

impl PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn default_code(mut self, value: impl Into<String>) -> Self {
        self.default_code = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn overridden(mut self, value: bool) -> Self {
        self.overridden = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesUpdateResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder::key)
    /// - [`description`](PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder::description)
    /// - [`default_code`](PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder::default_code)
    /// - [`account_code`](PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder::account_code)
    /// - [`overridden`](PostV1LedgerPostingRulesUpdateResponseRowsItemBuilder::overridden)
    pub fn build(self) -> Result<PostV1LedgerPostingRulesUpdateResponseRowsItem, BuildError> {
        Ok(PostV1LedgerPostingRulesUpdateResponseRowsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            default_code: self
                .default_code
                .ok_or_else(|| BuildError::missing_field("default_code"))?,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            overridden: self
                .overridden
                .ok_or_else(|| BuildError::missing_field("overridden"))?,
        })
    }
}
