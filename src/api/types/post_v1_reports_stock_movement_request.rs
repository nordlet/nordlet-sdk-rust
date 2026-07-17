pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockMovementRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl PostV1ReportsStockMovementRequest {
    pub fn builder() -> PostV1ReportsStockMovementRequestBuilder {
        <PostV1ReportsStockMovementRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockMovementRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    warehouse_id: Option<String>,
    item_id: Option<String>,
}

impl PostV1ReportsStockMovementRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockMovementRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsStockMovementRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsStockMovementRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsStockMovementRequest, BuildError> {
        Ok(PostV1ReportsStockMovementRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            warehouse_id: self.warehouse_id,
            item_id: self.item_id,
        })
    }
}
