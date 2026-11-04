pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsListCatalogRequest {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
}

impl ItemsUnitsListCatalogRequest {
    pub fn builder() -> ItemsUnitsListCatalogRequestBuilder {
        <ItemsUnitsListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsListCatalogRequestBuilder {
    item_id: Option<String>,
}

impl ItemsUnitsListCatalogRequestBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUnitsListCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](ItemsUnitsListCatalogRequestBuilder::item_id)
    pub fn build(self) -> Result<ItemsUnitsListCatalogRequest, BuildError> {
        Ok(ItemsUnitsListCatalogRequest {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
        })
    }
}
