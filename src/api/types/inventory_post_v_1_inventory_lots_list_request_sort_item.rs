pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryLotsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1InventoryLotsListRequestSortItemDir>,
}

impl PostV1InventoryLotsListRequestSortItem {
    pub fn builder() -> PostV1InventoryLotsListRequestSortItemBuilder {
        <PostV1InventoryLotsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryLotsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1InventoryLotsListRequestSortItemDir>,
}

impl PostV1InventoryLotsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1InventoryLotsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryLotsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryLotsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1InventoryLotsListRequestSortItem, BuildError> {
        Ok(PostV1InventoryLotsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
