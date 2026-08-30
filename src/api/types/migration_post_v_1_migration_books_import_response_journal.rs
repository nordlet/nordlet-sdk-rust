pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportResponseJournal {
    #[serde(default)]
    pub transactions: i64,
    #[serde(default)]
    pub entries: i64,
}

impl PostV1MigrationBooksImportResponseJournal {
    pub fn builder() -> PostV1MigrationBooksImportResponseJournalBuilder {
        <PostV1MigrationBooksImportResponseJournalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportResponseJournalBuilder {
    transactions: Option<i64>,
    entries: Option<i64>,
}

impl PostV1MigrationBooksImportResponseJournalBuilder {
    pub fn transactions(mut self, value: i64) -> Self {
        self.transactions = Some(value);
        self
    }

    pub fn entries(mut self, value: i64) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportResponseJournal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transactions`](PostV1MigrationBooksImportResponseJournalBuilder::transactions)
    /// - [`entries`](PostV1MigrationBooksImportResponseJournalBuilder::entries)
    pub fn build(self) -> Result<PostV1MigrationBooksImportResponseJournal, BuildError> {
        Ok(PostV1MigrationBooksImportResponseJournal {
            transactions: self
                .transactions
                .ok_or_else(|| BuildError::missing_field("transactions"))?,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
