pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1CatalogItemsListRequestSortItemDir>,
}

impl PostV1CatalogItemsListRequestSortItem {
    pub fn builder() -> PostV1CatalogItemsListRequestSortItemBuilder {
        <PostV1CatalogItemsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1CatalogItemsListRequestSortItemDir>,
}

impl PostV1CatalogItemsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1CatalogItemsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1CatalogItemsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1CatalogItemsListRequestSortItem, BuildError> {
        Ok(PostV1CatalogItemsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
