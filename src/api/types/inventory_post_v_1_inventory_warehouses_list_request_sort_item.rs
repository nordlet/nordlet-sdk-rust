pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryWarehousesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1InventoryWarehousesListRequestSortItemDir>,
}

impl PostV1InventoryWarehousesListRequestSortItem {
    pub fn builder() -> PostV1InventoryWarehousesListRequestSortItemBuilder {
        <PostV1InventoryWarehousesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryWarehousesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1InventoryWarehousesListRequestSortItemDir>,
}

impl PostV1InventoryWarehousesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1InventoryWarehousesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryWarehousesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryWarehousesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1InventoryWarehousesListRequestSortItem, BuildError> {
        Ok(PostV1InventoryWarehousesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
