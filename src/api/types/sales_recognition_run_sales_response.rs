pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecognitionRunSalesResponse {
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "runDate")]
    #[serde(default)]
    pub run_date: NaiveDate,
    pub trigger: RecognitionRunSalesResponseTrigger,
    #[serde(rename = "scheduleCount")]
    #[serde(default)]
    pub schedule_count: i64,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(default)]
    pub journal_transaction_id: String,
}

impl RecognitionRunSalesResponse {
    pub fn builder() -> RecognitionRunSalesResponseBuilder {
        <RecognitionRunSalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionRunSalesResponseBuilder {
    run_id: Option<String>,
    run_date: Option<NaiveDate>,
    trigger: Option<RecognitionRunSalesResponseTrigger>,
    schedule_count: Option<i64>,
    total_amount: Option<String>,
    journal_transaction_id: Option<String>,
}

impl RecognitionRunSalesResponseBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn run_date(mut self, value: NaiveDate) -> Self {
        self.run_date = Some(value);
        self
    }

    pub fn trigger(mut self, value: RecognitionRunSalesResponseTrigger) -> Self {
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

    /// Consumes the builder and constructs a [`RecognitionRunSalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](RecognitionRunSalesResponseBuilder::run_id)
    /// - [`run_date`](RecognitionRunSalesResponseBuilder::run_date)
    /// - [`trigger`](RecognitionRunSalesResponseBuilder::trigger)
    /// - [`schedule_count`](RecognitionRunSalesResponseBuilder::schedule_count)
    /// - [`total_amount`](RecognitionRunSalesResponseBuilder::total_amount)
    /// - [`journal_transaction_id`](RecognitionRunSalesResponseBuilder::journal_transaction_id)
    pub fn build(self) -> Result<RecognitionRunSalesResponse, BuildError> {
        Ok(RecognitionRunSalesResponse {
            run_id: self
                .run_id
                .ok_or_else(|| BuildError::missing_field("run_id"))?,
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
        })
    }
}
