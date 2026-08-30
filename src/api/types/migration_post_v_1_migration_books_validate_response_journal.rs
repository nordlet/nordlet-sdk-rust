pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateResponseJournal {
    #[serde(default)]
    pub transactions: i64,
    #[serde(default)]
    pub entries: i64,
}

impl PostV1MigrationBooksValidateResponseJournal {
    pub fn builder() -> PostV1MigrationBooksValidateResponseJournalBuilder {
        <PostV1MigrationBooksValidateResponseJournalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateResponseJournalBuilder {
    transactions: Option<i64>,
    entries: Option<i64>,
}

impl PostV1MigrationBooksValidateResponseJournalBuilder {
    pub fn transactions(mut self, value: i64) -> Self {
        self.transactions = Some(value);
        self
    }

    pub fn entries(mut self, value: i64) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateResponseJournal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transactions`](PostV1MigrationBooksValidateResponseJournalBuilder::transactions)
    /// - [`entries`](PostV1MigrationBooksValidateResponseJournalBuilder::entries)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateResponseJournal, BuildError> {
        Ok(PostV1MigrationBooksValidateResponseJournal {
            transactions: self
                .transactions
                .ok_or_else(|| BuildError::missing_field("transactions"))?,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
