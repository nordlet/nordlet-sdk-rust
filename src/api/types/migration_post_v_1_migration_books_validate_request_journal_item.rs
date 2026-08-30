pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1MigrationBooksValidateRequestJournalItem {
    #[serde(default)]
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default)]
    pub entries: Vec<PostV1MigrationBooksValidateRequestJournalItemEntriesItem>,
}

impl PostV1MigrationBooksValidateRequestJournalItem {
    pub fn builder() -> PostV1MigrationBooksValidateRequestJournalItemBuilder {
        <PostV1MigrationBooksValidateRequestJournalItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1MigrationBooksValidateRequestJournalItemBuilder {
    date: Option<String>,
    description: Option<String>,
    reference: Option<String>,
    entries: Option<Vec<PostV1MigrationBooksValidateRequestJournalItemEntriesItem>>,
}

impl PostV1MigrationBooksValidateRequestJournalItemBuilder {
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
        value: Vec<PostV1MigrationBooksValidateRequestJournalItemEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1MigrationBooksValidateRequestJournalItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1MigrationBooksValidateRequestJournalItemBuilder::date)
    /// - [`entries`](PostV1MigrationBooksValidateRequestJournalItemBuilder::entries)
    pub fn build(self) -> Result<PostV1MigrationBooksValidateRequestJournalItem, BuildError> {
        Ok(PostV1MigrationBooksValidateRequestJournalItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            reference: self.reference,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
