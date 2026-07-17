pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGeneralJournalResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(default)]
    pub entries: Vec<PostV1ReportsGeneralJournalResponseRowsItemEntriesItem>,
}

impl PostV1ReportsGeneralJournalResponseRowsItem {
    pub fn builder() -> PostV1ReportsGeneralJournalResponseRowsItemBuilder {
        <PostV1ReportsGeneralJournalResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGeneralJournalResponseRowsItemBuilder {
    id: Option<String>,
    date: Option<String>,
    description: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    entries: Option<Vec<PostV1ReportsGeneralJournalResponseRowsItemEntriesItem>>,
}

impl PostV1ReportsGeneralJournalResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    pub fn entries(
        mut self,
        value: Vec<PostV1ReportsGeneralJournalResponseRowsItemEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsGeneralJournalResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ReportsGeneralJournalResponseRowsItemBuilder::id)
    /// - [`date`](PostV1ReportsGeneralJournalResponseRowsItemBuilder::date)
    /// - [`entries`](PostV1ReportsGeneralJournalResponseRowsItemBuilder::entries)
    pub fn build(self) -> Result<PostV1ReportsGeneralJournalResponseRowsItem, BuildError> {
        Ok(PostV1ReportsGeneralJournalResponseRowsItem {
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
