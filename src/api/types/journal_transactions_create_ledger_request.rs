pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JournalTransactionsCreateLedgerRequest {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    pub entries: Vec<JournalTransactionsCreateLedgerRequestEntriesItem>,
}

impl JournalTransactionsCreateLedgerRequest {
    pub fn builder() -> JournalTransactionsCreateLedgerRequestBuilder {
        <JournalTransactionsCreateLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsCreateLedgerRequestBuilder {
    date: Option<NaiveDate>,
    description: Option<String>,
    currency: Option<String>,
    entries: Option<Vec<JournalTransactionsCreateLedgerRequestEntriesItem>>,
}

impl JournalTransactionsCreateLedgerRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn entries(
        mut self,
        value: Vec<JournalTransactionsCreateLedgerRequestEntriesItem>,
    ) -> Self {
        self.entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JournalTransactionsCreateLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](JournalTransactionsCreateLedgerRequestBuilder::date)
    /// - [`entries`](JournalTransactionsCreateLedgerRequestBuilder::entries)
    pub fn build(self) -> Result<JournalTransactionsCreateLedgerRequest, BuildError> {
        Ok(JournalTransactionsCreateLedgerRequest {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            currency: self.currency,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
        })
    }
}
