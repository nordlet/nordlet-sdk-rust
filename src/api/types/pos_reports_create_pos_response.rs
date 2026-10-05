pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportsCreatePosResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "reportNumber")]
    #[serde(default)]
    pub report_number: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "cashAmount")]
    #[serde(default)]
    pub cash_amount: String,
    #[serde(rename = "cardAmount")]
    #[serde(default)]
    pub card_amount: String,
    #[serde(rename = "cogsTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs_total: Option<String>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "vatLines")]
    #[serde(default)]
    pub vat_lines: Vec<ReportsCreatePosResponseVatLinesItem>,
}

impl ReportsCreatePosResponse {
    pub fn builder() -> ReportsCreatePosResponseBuilder {
        <ReportsCreatePosResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsCreatePosResponseBuilder {
    id: Option<String>,
    report_number: Option<String>,
    date: Option<NaiveDate>,
    device_id: Option<String>,
    warehouse_id: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
    cash_amount: Option<String>,
    card_amount: Option<String>,
    cogs_total: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    vat_lines: Option<Vec<ReportsCreatePosResponseVatLinesItem>>,
}

impl ReportsCreatePosResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn report_number(mut self, value: impl Into<String>) -> Self {
        self.report_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn cash_amount(mut self, value: impl Into<String>) -> Self {
        self.cash_amount = Some(value.into());
        self
    }

    pub fn card_amount(mut self, value: impl Into<String>) -> Self {
        self.card_amount = Some(value.into());
        self
    }

    pub fn cogs_total(mut self, value: impl Into<String>) -> Self {
        self.cogs_total = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn vat_lines(mut self, value: Vec<ReportsCreatePosResponseVatLinesItem>) -> Self {
        self.vat_lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportsCreatePosResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReportsCreatePosResponseBuilder::id)
    /// - [`report_number`](ReportsCreatePosResponseBuilder::report_number)
    /// - [`date`](ReportsCreatePosResponseBuilder::date)
    /// - [`net_total`](ReportsCreatePosResponseBuilder::net_total)
    /// - [`vat_total`](ReportsCreatePosResponseBuilder::vat_total)
    /// - [`gross_total`](ReportsCreatePosResponseBuilder::gross_total)
    /// - [`cash_amount`](ReportsCreatePosResponseBuilder::cash_amount)
    /// - [`card_amount`](ReportsCreatePosResponseBuilder::card_amount)
    /// - [`created_at`](ReportsCreatePosResponseBuilder::created_at)
    /// - [`vat_lines`](ReportsCreatePosResponseBuilder::vat_lines)
    pub fn build(self) -> Result<ReportsCreatePosResponse, BuildError> {
        Ok(ReportsCreatePosResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            report_number: self
                .report_number
                .ok_or_else(|| BuildError::missing_field("report_number"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            device_id: self.device_id,
            warehouse_id: self.warehouse_id,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            cash_amount: self
                .cash_amount
                .ok_or_else(|| BuildError::missing_field("cash_amount"))?,
            card_amount: self
                .card_amount
                .ok_or_else(|| BuildError::missing_field("card_amount"))?,
            cogs_total: self.cogs_total,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            vat_lines: self
                .vat_lines
                .ok_or_else(|| BuildError::missing_field("vat_lines"))?,
        })
    }
}
