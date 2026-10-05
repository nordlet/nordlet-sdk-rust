pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsSyncBankResponseAccountsItem {
    #[serde(rename = "feedAccountId")]
    #[serde(default)]
    pub feed_account_id: String,
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub fetched: i64,
}

impl FeedsSyncBankResponseAccountsItem {
    pub fn builder() -> FeedsSyncBankResponseAccountsItemBuilder {
        <FeedsSyncBankResponseAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsSyncBankResponseAccountsItemBuilder {
    feed_account_id: Option<String>,
    imported: Option<i64>,
    fetched: Option<i64>,
}

impl FeedsSyncBankResponseAccountsItemBuilder {
    pub fn feed_account_id(mut self, value: impl Into<String>) -> Self {
        self.feed_account_id = Some(value.into());
        self
    }

    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn fetched(mut self, value: i64) -> Self {
        self.fetched = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsSyncBankResponseAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feed_account_id`](FeedsSyncBankResponseAccountsItemBuilder::feed_account_id)
    /// - [`imported`](FeedsSyncBankResponseAccountsItemBuilder::imported)
    /// - [`fetched`](FeedsSyncBankResponseAccountsItemBuilder::fetched)
    pub fn build(self) -> Result<FeedsSyncBankResponseAccountsItem, BuildError> {
        Ok(FeedsSyncBankResponseAccountsItem {
            feed_account_id: self
                .feed_account_id
                .ok_or_else(|| BuildError::missing_field("feed_account_id"))?,
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            fetched: self
                .fetched
                .ok_or_else(|| BuildError::missing_field("fetched"))?,
        })
    }
}
