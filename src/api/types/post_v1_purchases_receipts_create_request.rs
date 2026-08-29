pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesReceiptsCreateRequest {
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "receiptDate")]
    #[serde(default)]
    pub receipt_date: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1PurchasesReceiptsCreateRequestLinesItem>,
}

impl PostV1PurchasesReceiptsCreateRequest {
    pub fn builder() -> PostV1PurchasesReceiptsCreateRequestBuilder {
        <PostV1PurchasesReceiptsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsCreateRequestBuilder {
    order_id: Option<String>,
    receipt_date: Option<String>,
    warehouse_id: Option<String>,
    notes: Option<String>,
    lines: Option<Vec<PostV1PurchasesReceiptsCreateRequestLinesItem>>,
}

impl PostV1PurchasesReceiptsCreateRequestBuilder {
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn receipt_date(mut self, value: impl Into<String>) -> Self {
        self.receipt_date = Some(value.into());
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

    pub fn lines(mut self, value: Vec<PostV1PurchasesReceiptsCreateRequestLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](PostV1PurchasesReceiptsCreateRequestBuilder::order_id)
    /// - [`receipt_date`](PostV1PurchasesReceiptsCreateRequestBuilder::receipt_date)
    /// - [`lines`](PostV1PurchasesReceiptsCreateRequestBuilder::lines)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsCreateRequest, BuildError> {
        Ok(PostV1PurchasesReceiptsCreateRequest {
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
