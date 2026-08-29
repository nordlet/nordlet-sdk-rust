pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsSyncResponse {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub imported: i64,
    #[serde(default)]
    pub skipped: i64,
    #[serde(default)]
    pub accounts: Vec<PostV1BankFeedsSyncResponseAccountsItem>,
}

impl PostV1BankFeedsSyncResponse {
    pub fn builder() -> PostV1BankFeedsSyncResponseBuilder {
        <PostV1BankFeedsSyncResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsSyncResponseBuilder {
    connection_id: Option<String>,
    imported: Option<i64>,
    skipped: Option<i64>,
    accounts: Option<Vec<PostV1BankFeedsSyncResponseAccountsItem>>,
}

impl PostV1BankFeedsSyncResponseBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn imported(mut self, value: i64) -> Self {
        self.imported = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    pub fn accounts(mut self, value: Vec<PostV1BankFeedsSyncResponseAccountsItem>) -> Self {
        self.accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsSyncResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](PostV1BankFeedsSyncResponseBuilder::connection_id)
    /// - [`imported`](PostV1BankFeedsSyncResponseBuilder::imported)
    /// - [`skipped`](PostV1BankFeedsSyncResponseBuilder::skipped)
    /// - [`accounts`](PostV1BankFeedsSyncResponseBuilder::accounts)
    pub fn build(self) -> Result<PostV1BankFeedsSyncResponse, BuildError> {
        Ok(PostV1BankFeedsSyncResponse {
            connection_id: self
                .connection_id
                .ok_or_else(|| BuildError::missing_field("connection_id"))?,
            imported: self
                .imported
                .ok_or_else(|| BuildError::missing_field("imported"))?,
            skipped: self
                .skipped
                .ok_or_else(|| BuildError::missing_field("skipped"))?,
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
        })
    }
}
