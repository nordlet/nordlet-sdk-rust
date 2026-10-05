pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BooksValidateMigrationRequestJournalItem {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default)]
    pub entries: Vec<BooksValidateMigrationRequestJournalItemEntriesItem>,
}

impl BooksValidateMigrationRequestJournalItem {
    pub fn builder() -> BooksValidateMigrationRequestJournalItemBuilder {
        <BooksValidateMigrationRequestJournalItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BooksValidateMigrationRequestJournalItemBuilder {
    date: Option<NaiveDate>,
    description: Option<String>,
    reference: Option<String>,
    entries: Option<Vec<BooksValidateMigrationRequestJournalItemEntriesItem>>,
}

impl BooksValidateMigrationRequestJournalItemBuilder {
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
        value: Vec<BooksValidateMigrationRequestJournalItemEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BooksValidateMigrationRequestJournalItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](BooksValidateMigrationRequestJournalItemBuilder::date)
    /// - [`entries`](BooksValidateMigrationRequestJournalItemBuilder::entries)
    pub fn build(self) -> Result<BooksValidateMigrationRequestJournalItem, BuildError> {
        Ok(BooksValidateMigrationRequestJournalItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            reference: self.reference,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
