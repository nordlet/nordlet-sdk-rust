pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PurchasesReceiptsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "receiptNumber")]
    #[serde(default)]
    pub receipt_number: String,
    #[serde(rename = "receiptDate")]
    #[serde(default)]
    pub receipt_date: String,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PurchasesReceiptsListResponseRowsItem {
    pub fn builder() -> PostV1PurchasesReceiptsListResponseRowsItemBuilder {
        <PostV1PurchasesReceiptsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PurchasesReceiptsListResponseRowsItemBuilder {
    id: Option<String>,
    order_id: Option<String>,
    receipt_number: Option<String>,
    receipt_date: Option<String>,
    warehouse_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1PurchasesReceiptsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn receipt_number(mut self, value: impl Into<String>) -> Self {
        self.receipt_number = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PurchasesReceiptsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PurchasesReceiptsListResponseRowsItemBuilder::id)
    /// - [`order_id`](PostV1PurchasesReceiptsListResponseRowsItemBuilder::order_id)
    /// - [`receipt_number`](PostV1PurchasesReceiptsListResponseRowsItemBuilder::receipt_number)
    /// - [`receipt_date`](PostV1PurchasesReceiptsListResponseRowsItemBuilder::receipt_date)
    /// - [`warehouse_id`](PostV1PurchasesReceiptsListResponseRowsItemBuilder::warehouse_id)
    /// - [`created_at`](PostV1PurchasesReceiptsListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1PurchasesReceiptsListResponseRowsItem, BuildError> {
        Ok(PostV1PurchasesReceiptsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            receipt_number: self
                .receipt_number
                .ok_or_else(|| BuildError::missing_field("receipt_number"))?,
            receipt_date: self
                .receipt_date
                .ok_or_else(|| BuildError::missing_field("receipt_date"))?,
            warehouse_id: self
                .warehouse_id
                .ok_or_else(|| BuildError::missing_field("warehouse_id"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
