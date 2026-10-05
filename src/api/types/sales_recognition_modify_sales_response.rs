pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecognitionModifySalesResponse {
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    pub approach: RecognitionModifySalesResponseApproach,
    #[serde(rename = "cancelledCount")]
    #[serde(default)]
    pub cancelled_count: i64,
    #[serde(rename = "newPendingCount")]
    #[serde(default)]
    pub new_pending_count: i64,
    #[serde(rename = "catchUpAmount")]
    #[serde(default)]
    pub catch_up_amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "newEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_end_date: Option<NaiveDate>,
}

impl RecognitionModifySalesResponse {
    pub fn builder() -> RecognitionModifySalesResponseBuilder {
        <RecognitionModifySalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionModifySalesResponseBuilder {
    invoice_line_id: Option<String>,
    approach: Option<RecognitionModifySalesResponseApproach>,
    cancelled_count: Option<i64>,
    new_pending_count: Option<i64>,
    catch_up_amount: Option<String>,
    journal_transaction_id: Option<String>,
    new_end_date: Option<NaiveDate>,
}

impl RecognitionModifySalesResponseBuilder {
    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn approach(mut self, value: RecognitionModifySalesResponseApproach) -> Self {
        self.approach = Some(value);
        self
    }

    pub fn cancelled_count(mut self, value: i64) -> Self {
        self.cancelled_count = Some(value);
        self
    }

    pub fn new_pending_count(mut self, value: i64) -> Self {
        self.new_pending_count = Some(value);
        self
    }

    pub fn catch_up_amount(mut self, value: impl Into<String>) -> Self {
        self.catch_up_amount = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn new_end_date(mut self, value: NaiveDate) -> Self {
        self.new_end_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionModifySalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_line_id`](RecognitionModifySalesResponseBuilder::invoice_line_id)
    /// - [`approach`](RecognitionModifySalesResponseBuilder::approach)
    /// - [`cancelled_count`](RecognitionModifySalesResponseBuilder::cancelled_count)
    /// - [`new_pending_count`](RecognitionModifySalesResponseBuilder::new_pending_count)
    /// - [`catch_up_amount`](RecognitionModifySalesResponseBuilder::catch_up_amount)
    pub fn build(self) -> Result<RecognitionModifySalesResponse, BuildError> {
        Ok(RecognitionModifySalesResponse {
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            approach: self
                .approach
                .ok_or_else(|| BuildError::missing_field("approach"))?,
            cancelled_count: self
                .cancelled_count
                .ok_or_else(|| BuildError::missing_field("cancelled_count"))?,
            new_pending_count: self
                .new_pending_count
                .ok_or_else(|| BuildError::missing_field("new_pending_count"))?,
            catch_up_amount: self
                .catch_up_amount
                .ok_or_else(|| BuildError::missing_field("catch_up_amount"))?,
            journal_transaction_id: self.journal_transaction_id,
            new_end_date: self.new_end_date,
        })
    }
}
