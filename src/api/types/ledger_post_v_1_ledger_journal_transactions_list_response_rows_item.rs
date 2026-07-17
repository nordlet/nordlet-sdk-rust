pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1LedgerJournalTransactionsListResponseRowsItem {
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
    pub status: PostV1LedgerJournalTransactionsListResponseRowsItemStatus,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "postedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<String>,
}

impl PostV1LedgerJournalTransactionsListResponseRowsItem {
    pub fn builder() -> PostV1LedgerJournalTransactionsListResponseRowsItemBuilder {
        <PostV1LedgerJournalTransactionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerJournalTransactionsListResponseRowsItemBuilder {
    id: Option<String>,
    date: Option<String>,
    description: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    status: Option<PostV1LedgerJournalTransactionsListResponseRowsItemStatus>,
    created_at: Option<String>,
    posted_at: Option<String>,
}

impl PostV1LedgerJournalTransactionsListResponseRowsItemBuilder {
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

    pub fn status(
        mut self,
        value: PostV1LedgerJournalTransactionsListResponseRowsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn posted_at(mut self, value: impl Into<String>) -> Self {
        self.posted_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerJournalTransactionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerJournalTransactionsListResponseRowsItemBuilder::id)
    /// - [`date`](PostV1LedgerJournalTransactionsListResponseRowsItemBuilder::date)
    /// - [`status`](PostV1LedgerJournalTransactionsListResponseRowsItemBuilder::status)
    /// - [`created_at`](PostV1LedgerJournalTransactionsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1LedgerJournalTransactionsListResponseRowsItem, BuildError> {
        Ok(PostV1LedgerJournalTransactionsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            document_type: self.document_type,
            document_id: self.document_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            posted_at: self.posted_at,
        })
    }
}
