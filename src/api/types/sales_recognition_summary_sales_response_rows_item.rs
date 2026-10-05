pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecognitionSummarySalesResponseRowsItem {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "invoiceFullNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_full_number: Option<String>,
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    #[serde(rename = "lineDescription")]
    #[serde(default)]
    pub line_description: String,
    pub method: RecognitionSummarySalesResponseRowsItemMethod,
    #[serde(rename = "deferredTotal")]
    #[serde(default)]
    pub deferred_total: String,
    #[serde(rename = "recognizedToDate")]
    #[serde(default)]
    pub recognized_to_date: NaiveDate,
    #[serde(default)]
    pub remaining: String,
    #[serde(rename = "pendingCount")]
    #[serde(default)]
    pub pending_count: i64,
    #[serde(rename = "nextScheduleDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_schedule_date: Option<NaiveDate>,
}

impl RecognitionSummarySalesResponseRowsItem {
    pub fn builder() -> RecognitionSummarySalesResponseRowsItemBuilder {
        <RecognitionSummarySalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSummarySalesResponseRowsItemBuilder {
    invoice_id: Option<String>,
    invoice_full_number: Option<String>,
    invoice_line_id: Option<String>,
    line_description: Option<String>,
    method: Option<RecognitionSummarySalesResponseRowsItemMethod>,
    deferred_total: Option<String>,
    recognized_to_date: Option<NaiveDate>,
    remaining: Option<String>,
    pending_count: Option<i64>,
    next_schedule_date: Option<NaiveDate>,
}

impl RecognitionSummarySalesResponseRowsItemBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn invoice_full_number(mut self, value: impl Into<String>) -> Self {
        self.invoice_full_number = Some(value.into());
        self
    }

    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn line_description(mut self, value: impl Into<String>) -> Self {
        self.line_description = Some(value.into());
        self
    }

    pub fn method(mut self, value: RecognitionSummarySalesResponseRowsItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn deferred_total(mut self, value: impl Into<String>) -> Self {
        self.deferred_total = Some(value.into());
        self
    }

    pub fn recognized_to_date(mut self, value: NaiveDate) -> Self {
        self.recognized_to_date = Some(value);
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn pending_count(mut self, value: i64) -> Self {
        self.pending_count = Some(value);
        self
    }

    pub fn next_schedule_date(mut self, value: NaiveDate) -> Self {
        self.next_schedule_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSummarySalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](RecognitionSummarySalesResponseRowsItemBuilder::invoice_id)
    /// - [`invoice_line_id`](RecognitionSummarySalesResponseRowsItemBuilder::invoice_line_id)
    /// - [`line_description`](RecognitionSummarySalesResponseRowsItemBuilder::line_description)
    /// - [`method`](RecognitionSummarySalesResponseRowsItemBuilder::method)
    /// - [`deferred_total`](RecognitionSummarySalesResponseRowsItemBuilder::deferred_total)
    /// - [`recognized_to_date`](RecognitionSummarySalesResponseRowsItemBuilder::recognized_to_date)
    /// - [`remaining`](RecognitionSummarySalesResponseRowsItemBuilder::remaining)
    /// - [`pending_count`](RecognitionSummarySalesResponseRowsItemBuilder::pending_count)
    pub fn build(self) -> Result<RecognitionSummarySalesResponseRowsItem, BuildError> {
        Ok(RecognitionSummarySalesResponseRowsItem {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            invoice_full_number: self.invoice_full_number,
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            line_description: self
                .line_description
                .ok_or_else(|| BuildError::missing_field("line_description"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            deferred_total: self
                .deferred_total
                .ok_or_else(|| BuildError::missing_field("deferred_total"))?,
            recognized_to_date: self
                .recognized_to_date
                .ok_or_else(|| BuildError::missing_field("recognized_to_date"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            pending_count: self
                .pending_count
                .ok_or_else(|| BuildError::missing_field("pending_count"))?,
            next_schedule_date: self.next_schedule_date,
        })
    }
}
