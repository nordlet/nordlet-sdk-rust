pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkKrGenerateResponseCounts {
    #[serde(default)]
    pub accounts: i64,
    #[serde(rename = "journalRows")]
    #[serde(default)]
    pub journal_rows: i64,
    #[serde(rename = "entryRows")]
    #[serde(default)]
    pub entry_rows: i64,
}

impl PostV1DeclarationsPlJpkKrGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder {
        <PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder {
    accounts: Option<i64>,
    journal_rows: Option<i64>,
    entry_rows: Option<i64>,
}

impl PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder {
    pub fn accounts(mut self, value: i64) -> Self {
        self.accounts = Some(value);
        self
    }

    pub fn journal_rows(mut self, value: i64) -> Self {
        self.journal_rows = Some(value);
        self
    }

    pub fn entry_rows(mut self, value: i64) -> Self {
        self.entry_rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkKrGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accounts`](PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder::accounts)
    /// - [`journal_rows`](PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder::journal_rows)
    /// - [`entry_rows`](PostV1DeclarationsPlJpkKrGenerateResponseCountsBuilder::entry_rows)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkKrGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsPlJpkKrGenerateResponseCounts {
            accounts: self
                .accounts
                .ok_or_else(|| BuildError::missing_field("accounts"))?,
            journal_rows: self
                .journal_rows
                .ok_or_else(|| BuildError::missing_field("journal_rows"))?,
            entry_rows: self
                .entry_rows
                .ok_or_else(|| BuildError::missing_field("entry_rows"))?,
        })
    }
}
