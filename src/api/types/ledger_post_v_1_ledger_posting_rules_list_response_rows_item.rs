pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerPostingRulesListResponseRowsItem {
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

impl PostV1LedgerPostingRulesListResponseRowsItem {
    pub fn builder() -> PostV1LedgerPostingRulesListResponseRowsItemBuilder {
        <PostV1LedgerPostingRulesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerPostingRulesListResponseRowsItemBuilder {
    key: Option<String>,
    description: Option<String>,
    default_code: Option<String>,
    account_code: Option<String>,
    overridden: Option<bool>,
}

impl PostV1LedgerPostingRulesListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1LedgerPostingRulesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostV1LedgerPostingRulesListResponseRowsItemBuilder::key)
    /// - [`description`](PostV1LedgerPostingRulesListResponseRowsItemBuilder::description)
    /// - [`default_code`](PostV1LedgerPostingRulesListResponseRowsItemBuilder::default_code)
    /// - [`account_code`](PostV1LedgerPostingRulesListResponseRowsItemBuilder::account_code)
    /// - [`overridden`](PostV1LedgerPostingRulesListResponseRowsItemBuilder::overridden)
    pub fn build(self) -> Result<PostV1LedgerPostingRulesListResponseRowsItem, BuildError> {
        Ok(PostV1LedgerPostingRulesListResponseRowsItem {
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
