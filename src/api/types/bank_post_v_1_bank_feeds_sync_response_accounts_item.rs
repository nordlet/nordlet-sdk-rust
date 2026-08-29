pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsSyncResponseAccountsItem {
    #[serde(rename = "feedAccountId")]
    #[serde(default)]
    pub feed_account_id: String,
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub fetched: i64,
}

impl PostV1BankFeedsSyncResponseAccountsItem {
    pub fn builder() -> PostV1BankFeedsSyncResponseAccountsItemBuilder {
        <PostV1BankFeedsSyncResponseAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsSyncResponseAccountsItemBuilder {
    feed_account_id: Option<String>,
    imported: Option<i64>,
    fetched: Option<i64>,
}

impl PostV1BankFeedsSyncResponseAccountsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1BankFeedsSyncResponseAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feed_account_id`](PostV1BankFeedsSyncResponseAccountsItemBuilder::feed_account_id)
    /// - [`imported`](PostV1BankFeedsSyncResponseAccountsItemBuilder::imported)
    /// - [`fetched`](PostV1BankFeedsSyncResponseAccountsItemBuilder::fetched)
    pub fn build(self) -> Result<PostV1BankFeedsSyncResponseAccountsItem, BuildError> {
        Ok(PostV1BankFeedsSyncResponseAccountsItem {
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
