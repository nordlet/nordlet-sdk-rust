pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsListCatalogRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ItemsListCatalogRequestSortItemDir>,
}

impl ItemsListCatalogRequestSortItem {
    pub fn builder() -> ItemsListCatalogRequestSortItemBuilder {
        <ItemsListCatalogRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsListCatalogRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ItemsListCatalogRequestSortItemDir>,
}

impl ItemsListCatalogRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ItemsListCatalogRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsListCatalogRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ItemsListCatalogRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ItemsListCatalogRequestSortItem, BuildError> {
        Ok(ItemsListCatalogRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
