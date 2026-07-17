pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockLevelsResponse {
    #[serde(default)]
    pub rows: Vec<PostV1InventoryStockLevelsResponseRowsItem>,
}

impl PostV1InventoryStockLevelsResponse {
    pub fn builder() -> PostV1InventoryStockLevelsResponseBuilder {
        <PostV1InventoryStockLevelsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockLevelsResponseBuilder {
    rows: Option<Vec<PostV1InventoryStockLevelsResponseRowsItem>>,
}

impl PostV1InventoryStockLevelsResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1InventoryStockLevelsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockLevelsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1InventoryStockLevelsResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1InventoryStockLevelsResponse, BuildError> {
        Ok(PostV1InventoryStockLevelsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
