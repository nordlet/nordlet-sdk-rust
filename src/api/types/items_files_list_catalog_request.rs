pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsFilesListCatalogRequest {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
}

impl ItemsFilesListCatalogRequest {
    pub fn builder() -> ItemsFilesListCatalogRequestBuilder {
        <ItemsFilesListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsFilesListCatalogRequestBuilder {
    item_id: Option<String>,
}

impl ItemsFilesListCatalogRequestBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsFilesListCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](ItemsFilesListCatalogRequestBuilder::item_id)
    pub fn build(self) -> Result<ItemsFilesListCatalogRequest, BuildError> {
        Ok(ItemsFilesListCatalogRequest {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
        })
    }
}
