pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecognitionRunsListSalesResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "runDate")]
    #[serde(default)]
    pub run_date: NaiveDate,
    pub trigger: RecognitionRunsListSalesResponseRowsItemTrigger,
    #[serde(rename = "scheduleCount")]
    #[serde(default)]
    pub schedule_count: i64,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(default)]
    pub journal_transaction_id: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl RecognitionRunsListSalesResponseRowsItem {
    pub fn builder() -> RecognitionRunsListSalesResponseRowsItemBuilder {
        <RecognitionRunsListSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionRunsListSalesResponseRowsItemBuilder {
    id: Option<String>,
    run_date: Option<NaiveDate>,
    trigger: Option<RecognitionRunsListSalesResponseRowsItemTrigger>,
    schedule_count: Option<i64>,
    total_amount: Option<String>,
    journal_transaction_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl RecognitionRunsListSalesResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn run_date(mut self, value: NaiveDate) -> Self {
        self.run_date = Some(value);
        self
    }

    pub fn trigger(mut self, value: RecognitionRunsListSalesResponseRowsItemTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    pub fn schedule_count(mut self, value: i64) -> Self {
        self.schedule_count = Some(value);
        self
    }

    pub fn total_amount(mut self, value: impl Into<String>) -> Self {
        self.total_amount = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionRunsListSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecognitionRunsListSalesResponseRowsItemBuilder::id)
    /// - [`run_date`](RecognitionRunsListSalesResponseRowsItemBuilder::run_date)
    /// - [`trigger`](RecognitionRunsListSalesResponseRowsItemBuilder::trigger)
    /// - [`schedule_count`](RecognitionRunsListSalesResponseRowsItemBuilder::schedule_count)
    /// - [`total_amount`](RecognitionRunsListSalesResponseRowsItemBuilder::total_amount)
    /// - [`journal_transaction_id`](RecognitionRunsListSalesResponseRowsItemBuilder::journal_transaction_id)
    /// - [`created_at`](RecognitionRunsListSalesResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<RecognitionRunsListSalesResponseRowsItem, BuildError> {
        Ok(RecognitionRunsListSalesResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            run_date: self
                .run_date
                .ok_or_else(|| BuildError::missing_field("run_date"))?,
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
            schedule_count: self
                .schedule_count
                .ok_or_else(|| BuildError::missing_field("schedule_count"))?,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            journal_transaction_id: self
                .journal_transaction_id
                .ok_or_else(|| BuildError::missing_field("journal_transaction_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
