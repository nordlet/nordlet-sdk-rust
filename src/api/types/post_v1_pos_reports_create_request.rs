pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PosReportsCreateRequest {
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
    #[serde(rename = "vatLines")]
    #[serde(default)]
    pub vat_lines: Vec<PostV1PosReportsCreateRequestVatLinesItem>,
    #[serde(rename = "cashAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_amount: Option<String>,
    #[serde(rename = "cardAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_amount: Option<String>,
    #[serde(rename = "itemLines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_lines: Option<Vec<PostV1PosReportsCreateRequestItemLinesItem>>,
    #[serde(rename = "cashAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_account_code: Option<String>,
    #[serde(rename = "cardAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_account_code: Option<String>,
    #[serde(rename = "revenueAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_account_code: Option<String>,
    #[serde(rename = "vatAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_account_code: Option<String>,
    #[serde(rename = "cogsAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cogs_account_code: Option<String>,
    #[serde(rename = "inventoryAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1PosReportsCreateRequest {
    pub fn builder() -> PostV1PosReportsCreateRequestBuilder {
        <PostV1PosReportsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosReportsCreateRequestBuilder {
    report_number: Option<String>,
    date: Option<String>,
    device_id: Option<String>,
    warehouse_id: Option<String>,
    vat_lines: Option<Vec<PostV1PosReportsCreateRequestVatLinesItem>>,
    cash_amount: Option<String>,
    card_amount: Option<String>,
    item_lines: Option<Vec<PostV1PosReportsCreateRequestItemLinesItem>>,
    cash_account_code: Option<String>,
    card_account_code: Option<String>,
    revenue_account_code: Option<String>,
    vat_account_code: Option<String>,
    cogs_account_code: Option<String>,
    inventory_account_code: Option<String>,
    notes: Option<String>,
}

impl PostV1PosReportsCreateRequestBuilder {
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

    pub fn vat_lines(mut self, value: Vec<PostV1PosReportsCreateRequestVatLinesItem>) -> Self {
        self.vat_lines = Some(value);
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

    pub fn item_lines(mut self, value: Vec<PostV1PosReportsCreateRequestItemLinesItem>) -> Self {
        self.item_lines = Some(value);
        self
    }

    pub fn cash_account_code(mut self, value: impl Into<String>) -> Self {
        self.cash_account_code = Some(value.into());
        self
    }

    pub fn card_account_code(mut self, value: impl Into<String>) -> Self {
        self.card_account_code = Some(value.into());
        self
    }

    pub fn revenue_account_code(mut self, value: impl Into<String>) -> Self {
        self.revenue_account_code = Some(value.into());
        self
    }

    pub fn vat_account_code(mut self, value: impl Into<String>) -> Self {
        self.vat_account_code = Some(value.into());
        self
    }

    pub fn cogs_account_code(mut self, value: impl Into<String>) -> Self {
        self.cogs_account_code = Some(value.into());
        self
    }

    pub fn inventory_account_code(mut self, value: impl Into<String>) -> Self {
        self.inventory_account_code = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosReportsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`report_number`](PostV1PosReportsCreateRequestBuilder::report_number)
    /// - [`date`](PostV1PosReportsCreateRequestBuilder::date)
    /// - [`vat_lines`](PostV1PosReportsCreateRequestBuilder::vat_lines)
    pub fn build(self) -> Result<PostV1PosReportsCreateRequest, BuildError> {
        Ok(PostV1PosReportsCreateRequest {
            report_number: self
                .report_number
                .ok_or_else(|| BuildError::missing_field("report_number"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            device_id: self.device_id,
            warehouse_id: self.warehouse_id,
            vat_lines: self
                .vat_lines
                .ok_or_else(|| BuildError::missing_field("vat_lines"))?,
            cash_amount: self.cash_amount,
            card_amount: self.card_amount,
            item_lines: self.item_lines,
            cash_account_code: self.cash_account_code,
            card_account_code: self.card_account_code,
            revenue_account_code: self.revenue_account_code,
            vat_account_code: self.vat_account_code,
            cogs_account_code: self.cogs_account_code,
            inventory_account_code: self.inventory_account_code,
            notes: self.notes,
        })
    }
}
