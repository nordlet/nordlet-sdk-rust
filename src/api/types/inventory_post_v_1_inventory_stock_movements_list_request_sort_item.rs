pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryStockMovementsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1InventoryStockMovementsListRequestSortItemDir>,
}

impl PostV1InventoryStockMovementsListRequestSortItem {
    pub fn builder() -> PostV1InventoryStockMovementsListRequestSortItemBuilder {
        <PostV1InventoryStockMovementsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryStockMovementsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1InventoryStockMovementsListRequestSortItemDir>,
}

impl PostV1InventoryStockMovementsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1InventoryStockMovementsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryStockMovementsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryStockMovementsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1InventoryStockMovementsListRequestSortItem, BuildError> {
        Ok(PostV1InventoryStockMovementsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
