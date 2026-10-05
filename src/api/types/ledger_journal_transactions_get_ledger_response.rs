pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JournalTransactionsGetLedgerResponse {
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
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    pub status: JournalTransactionsGetLedgerResponseStatus,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "postedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub posted_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    pub entries: Vec<JournalTransactionsGetLedgerResponseEntriesItem>,
}

impl JournalTransactionsGetLedgerResponse {
    pub fn builder() -> JournalTransactionsGetLedgerResponseBuilder {
        <JournalTransactionsGetLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsGetLedgerResponseBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    description: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    partner_id: Option<String>,
    status: Option<JournalTransactionsGetLedgerResponseStatus>,
    created_at: Option<DateTime<FixedOffset>>,
    posted_at: Option<DateTime<FixedOffset>>,
    entries: Option<Vec<JournalTransactionsGetLedgerResponseEntriesItem>>,
}

impl JournalTransactionsGetLedgerResponseBuilder {
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: JournalTransactionsGetLedgerResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn posted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.posted_at = Some(value);
        self
    }

    pub fn entries(mut self, value: Vec<JournalTransactionsGetLedgerResponseEntriesItem>) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JournalTransactionsGetLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JournalTransactionsGetLedgerResponseBuilder::id)
    /// - [`date`](JournalTransactionsGetLedgerResponseBuilder::date)
    /// - [`status`](JournalTransactionsGetLedgerResponseBuilder::status)
    /// - [`created_at`](JournalTransactionsGetLedgerResponseBuilder::created_at)
    /// - [`entries`](JournalTransactionsGetLedgerResponseBuilder::entries)
    pub fn build(self) -> Result<JournalTransactionsGetLedgerResponse, BuildError> {
        Ok(JournalTransactionsGetLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            document_type: self.document_type,
            document_id: self.document_id,
            partner_id: self.partner_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            posted_at: self.posted_at,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
