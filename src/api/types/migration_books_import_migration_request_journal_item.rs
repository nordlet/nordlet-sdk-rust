pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksImportMigrationRequestJournalItem {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default)]
    pub entries: Vec<BooksImportMigrationRequestJournalItemEntriesItem>,
}

impl BooksImportMigrationRequestJournalItem {
    pub fn builder() -> BooksImportMigrationRequestJournalItemBuilder {
        <BooksImportMigrationRequestJournalItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksImportMigrationRequestJournalItemBuilder {
    date: Option<NaiveDate>,
    description: Option<String>,
    reference: Option<String>,
    entries: Option<Vec<BooksImportMigrationRequestJournalItemEntriesItem>>,
}

impl BooksImportMigrationRequestJournalItemBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn entries(
        mut self,
        value: Vec<BooksImportMigrationRequestJournalItemEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksImportMigrationRequestJournalItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](BooksImportMigrationRequestJournalItemBuilder::date)
    /// - [`entries`](BooksImportMigrationRequestJournalItemBuilder::entries)
    pub fn build(self) -> Result<BooksImportMigrationRequestJournalItem, BuildError> {
        Ok(BooksImportMigrationRequestJournalItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            reference: self.reference,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
