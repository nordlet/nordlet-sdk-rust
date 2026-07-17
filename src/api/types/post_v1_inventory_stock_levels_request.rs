pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockLevelsRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl PostV1InventoryStockLevelsRequest {
    pub fn builder() -> PostV1InventoryStockLevelsRequestBuilder {
        <PostV1InventoryStockLevelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockLevelsRequestBuilder {
    warehouse_id: Option<String>,
    item_id: Option<String>,
}

impl PostV1InventoryStockLevelsRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockLevelsRequest`].
    pub fn build(self) -> Result<PostV1InventoryStockLevelsRequest, BuildError> {
        Ok(PostV1InventoryStockLevelsRequest {
            warehouse_id: self.warehouse_id,
            item_id: self.item_id,
        })
    }
}
