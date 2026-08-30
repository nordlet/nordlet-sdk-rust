pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksImportRequestJournalItem {
    #[serde(default)]
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default)]
    pub entries: Vec<PostV1MigrationBooksImportRequestJournalItemEntriesItem>,
}

impl PostV1MigrationBooksImportRequestJournalItem {
    pub fn builder() -> PostV1MigrationBooksImportRequestJournalItemBuilder {
        <PostV1MigrationBooksImportRequestJournalItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksImportRequestJournalItemBuilder {
    date: Option<String>,
    description: Option<String>,
    reference: Option<String>,
    entries: Option<Vec<PostV1MigrationBooksImportRequestJournalItemEntriesItem>>,
}

impl PostV1MigrationBooksImportRequestJournalItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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
        value: Vec<PostV1MigrationBooksImportRequestJournalItemEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksImportRequestJournalItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1MigrationBooksImportRequestJournalItemBuilder::date)
    /// - [`entries`](PostV1MigrationBooksImportRequestJournalItemBuilder::entries)
    pub fn build(self) -> Result<PostV1MigrationBooksImportRequestJournalItem, BuildError> {
        Ok(PostV1MigrationBooksImportRequestJournalItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            reference: self.reference,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
