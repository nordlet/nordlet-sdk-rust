pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ShiftsGetPosResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    pub status: ShiftsGetPosResponseStatus,
    #[serde(rename = "openingCash")]
    #[serde(default)]
    pub opening_cash: String,
    #[serde(rename = "countedCash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counted_cash: Option<String>,
    #[serde(rename = "receiptCount")]
    #[serde(default)]
    pub receipt_count: i64,
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(rename = "openedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub opened_at: DateTime<FixedOffset>,
    #[serde(rename = "closedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub closed_at: Option<DateTime<FixedOffset>>,
}

impl ShiftsGetPosResponse {
    pub fn builder() -> ShiftsGetPosResponseBuilder {
        <ShiftsGetPosResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShiftsGetPosResponseBuilder {
    id: Option<String>,
    device_id: Option<String>,
    warehouse_id: Option<String>,
    status: Option<ShiftsGetPosResponseStatus>,
    opening_cash: Option<String>,
    counted_cash: Option<String>,
    receipt_count: Option<i64>,
    report_id: Option<String>,
    opened_at: Option<DateTime<FixedOffset>>,
    closed_at: Option<DateTime<FixedOffset>>,
}

impl ShiftsGetPosResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn status(mut self, value: ShiftsGetPosResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn opening_cash(mut self, value: impl Into<String>) -> Self {
        self.opening_cash = Some(value.into());
        self
    }

    pub fn counted_cash(mut self, value: impl Into<String>) -> Self {
        self.counted_cash = Some(value.into());
        self
    }

    pub fn receipt_count(mut self, value: i64) -> Self {
        self.receipt_count = Some(value);
        self
    }

    pub fn report_id(mut self, value: impl Into<String>) -> Self {
        self.report_id = Some(value.into());
        self
    }

    pub fn opened_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.opened_at = Some(value);
        self
    }

    pub fn closed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.closed_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShiftsGetPosResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ShiftsGetPosResponseBuilder::id)
    /// - [`device_id`](ShiftsGetPosResponseBuilder::device_id)
    /// - [`status`](ShiftsGetPosResponseBuilder::status)
    /// - [`opening_cash`](ShiftsGetPosResponseBuilder::opening_cash)
    /// - [`receipt_count`](ShiftsGetPosResponseBuilder::receipt_count)
    /// - [`opened_at`](ShiftsGetPosResponseBuilder::opened_at)
    pub fn build(self) -> Result<ShiftsGetPosResponse, BuildError> {
        Ok(ShiftsGetPosResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            device_id: self
                .device_id
                .ok_or_else(|| BuildError::missing_field("device_id"))?,
            warehouse_id: self.warehouse_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            opening_cash: self
                .opening_cash
                .ok_or_else(|| BuildError::missing_field("opening_cash"))?,
            counted_cash: self.counted_cash,
            receipt_count: self
                .receipt_count
                .ok_or_else(|| BuildError::missing_field("receipt_count"))?,
            report_id: self.report_id,
            opened_at: self
                .opened_at
                .ok_or_else(|| BuildError::missing_field("opened_at"))?,
            closed_at: self.closed_at,
        })
    }
}
