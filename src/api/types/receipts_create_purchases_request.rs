pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsCreatePurchasesRequest {
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "receiptDate")]
    #[serde(default)]
    pub receipt_date: NaiveDate,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<ReceiptsCreatePurchasesRequestLinesItem>,
}

impl ReceiptsCreatePurchasesRequest {
    pub fn builder() -> ReceiptsCreatePurchasesRequestBuilder {
        <ReceiptsCreatePurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsCreatePurchasesRequestBuilder {
    order_id: Option<String>,
    receipt_date: Option<NaiveDate>,
    warehouse_id: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<ReceiptsCreatePurchasesRequestLinesItem>>,
}

impl ReceiptsCreatePurchasesRequestBuilder {
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn receipt_date(mut self, value: NaiveDate) -> Self {
        self.receipt_date = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<ReceiptsCreatePurchasesRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsCreatePurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](ReceiptsCreatePurchasesRequestBuilder::order_id)
    /// - [`receipt_date`](ReceiptsCreatePurchasesRequestBuilder::receipt_date)
    /// - [`lines`](ReceiptsCreatePurchasesRequestBuilder::lines)
    pub fn build(self) -> Result<ReceiptsCreatePurchasesRequest, BuildError> {
        Ok(ReceiptsCreatePurchasesRequest {
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            receipt_date: self
                .receipt_date
                .ok_or_else(|| BuildError::missing_field("receipt_date"))?,
            warehouse_id: self.warehouse_id,
            notes: self.notes,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
