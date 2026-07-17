pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerJournalTransactionsCreateRequest {
    #[serde(default)]
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub entries: Vec<PostV1LedgerJournalTransactionsCreateRequestEntriesItem>,
}

impl PostV1LedgerJournalTransactionsCreateRequest {
    pub fn builder() -> PostV1LedgerJournalTransactionsCreateRequestBuilder {
        <PostV1LedgerJournalTransactionsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerJournalTransactionsCreateRequestBuilder {
    date: Option<String>,
    description: Option<String>,
    entries: Option<Vec<PostV1LedgerJournalTransactionsCreateRequestEntriesItem>>,
}

impl PostV1LedgerJournalTransactionsCreateRequestBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn entries(
        mut self,
        value: Vec<PostV1LedgerJournalTransactionsCreateRequestEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerJournalTransactionsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1LedgerJournalTransactionsCreateRequestBuilder::date)
    /// - [`entries`](PostV1LedgerJournalTransactionsCreateRequestBuilder::entries)
    pub fn build(self) -> Result<PostV1LedgerJournalTransactionsCreateRequest, BuildError> {
        Ok(PostV1LedgerJournalTransactionsCreateRequest {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
