pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RecognitionSchedulesListSalesResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "invoiceLineId")]
    #[serde(default)]
    pub invoice_line_id: String,
    pub method: RecognitionSchedulesListSalesResponseRowsItemMethod,
    pub status: RecognitionSchedulesListSalesResponseRowsItemStatus,
    #[serde(rename = "scheduleDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "recognizedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub recognized_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl RecognitionSchedulesListSalesResponseRowsItem {
    pub fn builder() -> RecognitionSchedulesListSalesResponseRowsItemBuilder {
        <RecognitionSchedulesListSalesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSchedulesListSalesResponseRowsItemBuilder {
    id: Option<String>,
    invoice_id: Option<String>,
    invoice_line_id: Option<String>,
    method: Option<RecognitionSchedulesListSalesResponseRowsItemMethod>,
    status: Option<RecognitionSchedulesListSalesResponseRowsItemStatus>,
    schedule_date: Option<NaiveDate>,
    description: Option<String>,
    amount: Option<String>,
    journal_transaction_id: Option<String>,
    recognized_at: Option<DateTime<FixedOffset>>,
    sort_order: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl RecognitionSchedulesListSalesResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn invoice_line_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_id = Some(value.into());
        self
    }

    pub fn method(mut self, value: RecognitionSchedulesListSalesResponseRowsItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn status(mut self, value: RecognitionSchedulesListSalesResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn schedule_date(mut self, value: NaiveDate) -> Self {
        self.schedule_date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn recognized_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.recognized_at = Some(value);
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSchedulesListSalesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecognitionSchedulesListSalesResponseRowsItemBuilder::id)
    /// - [`invoice_id`](RecognitionSchedulesListSalesResponseRowsItemBuilder::invoice_id)
    /// - [`invoice_line_id`](RecognitionSchedulesListSalesResponseRowsItemBuilder::invoice_line_id)
    /// - [`method`](RecognitionSchedulesListSalesResponseRowsItemBuilder::method)
    /// - [`status`](RecognitionSchedulesListSalesResponseRowsItemBuilder::status)
    /// - [`amount`](RecognitionSchedulesListSalesResponseRowsItemBuilder::amount)
    /// - [`sort_order`](RecognitionSchedulesListSalesResponseRowsItemBuilder::sort_order)
    /// - [`created_at`](RecognitionSchedulesListSalesResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<RecognitionSchedulesListSalesResponseRowsItem, BuildError> {
        Ok(RecognitionSchedulesListSalesResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            invoice_line_id: self
                .invoice_line_id
                .ok_or_else(|| BuildError::missing_field("invoice_line_id"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            schedule_date: self.schedule_date,
            description: self.description,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            journal_transaction_id: self.journal_transaction_id,
            recognized_at: self.recognized_at,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
