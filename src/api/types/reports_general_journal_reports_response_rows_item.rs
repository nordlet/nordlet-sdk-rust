pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneralJournalReportsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(default)]
    pub entries: Vec<GeneralJournalReportsResponseRowsItemEntriesItem>,
}

impl GeneralJournalReportsResponseRowsItem {
    pub fn builder() -> GeneralJournalReportsResponseRowsItemBuilder {
        <GeneralJournalReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralJournalReportsResponseRowsItemBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    description: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    entries: Option<Vec<GeneralJournalReportsResponseRowsItemEntriesItem>>,
}

impl GeneralJournalReportsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: impl Into<String>) -> Self {
        self.document_type = Some(value.into());
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn entries(mut self, value: Vec<GeneralJournalReportsResponseRowsItemEntriesItem>) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneralJournalReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GeneralJournalReportsResponseRowsItemBuilder::id)
    /// - [`date`](GeneralJournalReportsResponseRowsItemBuilder::date)
    /// - [`entries`](GeneralJournalReportsResponseRowsItemBuilder::entries)
    pub fn build(self) -> Result<GeneralJournalReportsResponseRowsItem, BuildError> {
        Ok(GeneralJournalReportsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            document_type: self.document_type,
            document_id: self.document_id,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
