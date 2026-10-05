pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockLevelsInventoryRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl StockLevelsInventoryRequest {
    pub fn builder() -> StockLevelsInventoryRequestBuilder {
        <StockLevelsInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockLevelsInventoryRequestBuilder {
    warehouse_id: Option<String>,
    item_id: Option<String>,
}

impl StockLevelsInventoryRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockLevelsInventoryRequest`].
    pub fn build(self) -> Result<StockLevelsInventoryRequest, BuildError> {
        Ok(StockLevelsInventoryRequest {
            warehouse_id: self.warehouse_id,
            item_id: self.item_id,
        })
    }
}
