pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1InventoryReorderRulesListRequestSortItemDir>,
}

impl PostV1InventoryReorderRulesListRequestSortItem {
    pub fn builder() -> PostV1InventoryReorderRulesListRequestSortItemBuilder {
        <PostV1InventoryReorderRulesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1InventoryReorderRulesListRequestSortItemDir>,
}

impl PostV1InventoryReorderRulesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1InventoryReorderRulesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1InventoryReorderRulesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesListRequestSortItem, BuildError> {
        Ok(PostV1InventoryReorderRulesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
