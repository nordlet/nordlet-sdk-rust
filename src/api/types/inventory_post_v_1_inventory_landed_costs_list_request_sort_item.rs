pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLandedCostsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1InventoryLandedCostsListRequestSortItemDir>,
}

impl PostV1InventoryLandedCostsListRequestSortItem {
    pub fn builder() -> PostV1InventoryLandedCostsListRequestSortItemBuilder {
        <PostV1InventoryLandedCostsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLandedCostsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1InventoryLandedCostsListRequestSortItemDir>,
}

impl PostV1InventoryLandedCostsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1InventoryLandedCostsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLandedCostsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryLandedCostsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1InventoryLandedCostsListRequestSortItem, BuildError> {
        Ok(PostV1InventoryLandedCostsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
