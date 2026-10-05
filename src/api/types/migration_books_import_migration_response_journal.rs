pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationResponseJournal {
    #[serde(default)]
    pub transactions: i64,
    #[serde(default)]
    pub entries: i64,
}

impl BooksImportMigrationResponseJournal {
    pub fn builder() -> BooksImportMigrationResponseJournalBuilder {
        <BooksImportMigrationResponseJournalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationResponseJournalBuilder {
    transactions: Option<i64>,
    entries: Option<i64>,
}

impl BooksImportMigrationResponseJournalBuilder {
    pub fn transactions(mut self, value: i64) -> Self {
        self.transactions = Some(value);
        self
    }

    pub fn entries(mut self, value: i64) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationResponseJournal`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transactions`](BooksImportMigrationResponseJournalBuilder::transactions)
    /// - [`entries`](BooksImportMigrationResponseJournalBuilder::entries)
    pub fn build(self) -> Result<BooksImportMigrationResponseJournal, BuildError> {
        Ok(BooksImportMigrationResponseJournal {
            transactions: self
                .transactions
                .ok_or_else(|| BuildError::missing_field("transactions"))?,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
