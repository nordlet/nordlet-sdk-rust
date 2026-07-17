pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "reportNumber")]
    #[serde(default)]
    pub report_number: String,
    #[serde(default)]
    pub date: String,
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
    pub created_at: String,
}

impl PostV1PosReportsListResponseRowsItem {
    pub fn builder() -> PostV1PosReportsListResponseRowsItemBuilder {
        <PostV1PosReportsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsListResponseRowsItemBuilder {
    id: Option<String>,
    report_number: Option<String>,
    date: Option<String>,
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
    created_at: Option<String>,
}

impl PostV1PosReportsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn report_number(mut self, value: impl Into<String>) -> Self {
        self.report_number = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosReportsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PosReportsListResponseRowsItemBuilder::id)
    /// - [`report_number`](PostV1PosReportsListResponseRowsItemBuilder::report_number)
    /// - [`date`](PostV1PosReportsListResponseRowsItemBuilder::date)
    /// - [`net_total`](PostV1PosReportsListResponseRowsItemBuilder::net_total)
    /// - [`vat_total`](PostV1PosReportsListResponseRowsItemBuilder::vat_total)
    /// - [`gross_total`](PostV1PosReportsListResponseRowsItemBuilder::gross_total)
    /// - [`cash_amount`](PostV1PosReportsListResponseRowsItemBuilder::cash_amount)
    /// - [`card_amount`](PostV1PosReportsListResponseRowsItemBuilder::card_amount)
    /// - [`created_at`](PostV1PosReportsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1PosReportsListResponseRowsItem, BuildError> {
        Ok(PostV1PosReportsListResponseRowsItem {
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
        })
    }
}
