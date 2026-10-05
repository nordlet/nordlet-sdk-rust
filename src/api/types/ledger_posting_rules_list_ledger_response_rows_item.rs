pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostingRulesListLedgerResponseRowsItem {
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

impl PostingRulesListLedgerResponseRowsItem {
    pub fn builder() -> PostingRulesListLedgerResponseRowsItemBuilder {
        <PostingRulesListLedgerResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostingRulesListLedgerResponseRowsItemBuilder {
    key: Option<String>,
    description: Option<String>,
    default_code: Option<String>,
    account_code: Option<String>,
    overridden: Option<bool>,
}

impl PostingRulesListLedgerResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostingRulesListLedgerResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PostingRulesListLedgerResponseRowsItemBuilder::key)
    /// - [`description`](PostingRulesListLedgerResponseRowsItemBuilder::description)
    /// - [`default_code`](PostingRulesListLedgerResponseRowsItemBuilder::default_code)
    /// - [`account_code`](PostingRulesListLedgerResponseRowsItemBuilder::account_code)
    /// - [`overridden`](PostingRulesListLedgerResponseRowsItemBuilder::overridden)
    pub fn build(self) -> Result<PostingRulesListLedgerResponseRowsItem, BuildError> {
        Ok(PostingRulesListLedgerResponseRowsItem {
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
