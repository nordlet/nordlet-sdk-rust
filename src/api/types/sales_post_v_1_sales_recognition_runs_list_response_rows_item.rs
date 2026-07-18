pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionRunsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "runDate")]
    #[serde(default)]
    pub run_date: String,
    pub trigger: PostV1SalesRecognitionRunsListResponseRowsItemTrigger,
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
    pub created_at: String,
}

impl PostV1SalesRecognitionRunsListResponseRowsItem {
    pub fn builder() -> PostV1SalesRecognitionRunsListResponseRowsItemBuilder {
        <PostV1SalesRecognitionRunsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionRunsListResponseRowsItemBuilder {
    id: Option<String>,
    run_date: Option<String>,
    trigger: Option<PostV1SalesRecognitionRunsListResponseRowsItemTrigger>,
    schedule_count: Option<i64>,
    total_amount: Option<String>,
    journal_transaction_id: Option<String>,
    created_at: Option<String>,
}

impl PostV1SalesRecognitionRunsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn run_date(mut self, value: impl Into<String>) -> Self {
        self.run_date = Some(value.into());
        self
    }

    pub fn trigger(mut self, value: PostV1SalesRecognitionRunsListResponseRowsItemTrigger) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionRunsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::id)
    /// - [`run_date`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::run_date)
    /// - [`trigger`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::trigger)
    /// - [`schedule_count`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::schedule_count)
    /// - [`total_amount`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::total_amount)
    /// - [`journal_transaction_id`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::journal_transaction_id)
    /// - [`created_at`](PostV1SalesRecognitionRunsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1SalesRecognitionRunsListResponseRowsItem, BuildError> {
        Ok(PostV1SalesRecognitionRunsListResponseRowsItem {
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
