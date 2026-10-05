pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JournalTransactionsCreateLedgerResponse {
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
    pub status: JournalTransactionsCreateLedgerResponseStatus,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "postedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub posted_at: Option<DateTime<FixedOffset>>,
}

impl JournalTransactionsCreateLedgerResponse {
    pub fn builder() -> JournalTransactionsCreateLedgerResponseBuilder {
        <JournalTransactionsCreateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsCreateLedgerResponseBuilder {
    id: Option<String>,
    date: Option<NaiveDate>,
    description: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    partner_id: Option<String>,
    status: Option<JournalTransactionsCreateLedgerResponseStatus>,
    created_at: Option<DateTime<FixedOffset>>,
    posted_at: Option<DateTime<FixedOffset>>,
}

impl JournalTransactionsCreateLedgerResponseBuilder {
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

    pub fn status(mut self, value: JournalTransactionsCreateLedgerResponseStatus) -> Self {
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

    /// Consumes the builder and constructs a [`JournalTransactionsCreateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JournalTransactionsCreateLedgerResponseBuilder::id)
    /// - [`date`](JournalTransactionsCreateLedgerResponseBuilder::date)
    /// - [`status`](JournalTransactionsCreateLedgerResponseBuilder::status)
    /// - [`created_at`](JournalTransactionsCreateLedgerResponseBuilder::created_at)
    pub fn build(self) -> Result<JournalTransactionsCreateLedgerResponse, BuildError> {
        Ok(JournalTransactionsCreateLedgerResponse {
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
        })
    }
}
