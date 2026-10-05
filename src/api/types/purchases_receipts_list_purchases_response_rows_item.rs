pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsListPurchasesResponseRowsItem {
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
    pub receipt_date: NaiveDate,
    #[serde(rename = "warehouseId")]
    #[serde(default)]
    pub warehouse_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ReceiptsListPurchasesResponseRowsItem {
    pub fn builder() -> ReceiptsListPurchasesResponseRowsItemBuilder {
        <ReceiptsListPurchasesResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPurchasesResponseRowsItemBuilder {
    id: Option<String>,
    order_id: Option<String>,
    receipt_number: Option<String>,
    receipt_date: Option<NaiveDate>,
    warehouse_id: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ReceiptsListPurchasesResponseRowsItemBuilder {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsListPurchasesResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReceiptsListPurchasesResponseRowsItemBuilder::id)
    /// - [`order_id`](ReceiptsListPurchasesResponseRowsItemBuilder::order_id)
    /// - [`receipt_number`](ReceiptsListPurchasesResponseRowsItemBuilder::receipt_number)
    /// - [`receipt_date`](ReceiptsListPurchasesResponseRowsItemBuilder::receipt_date)
    /// - [`warehouse_id`](ReceiptsListPurchasesResponseRowsItemBuilder::warehouse_id)
    /// - [`created_at`](ReceiptsListPurchasesResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<ReceiptsListPurchasesResponseRowsItem, BuildError> {
        Ok(ReceiptsListPurchasesResponseRowsItem {
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
