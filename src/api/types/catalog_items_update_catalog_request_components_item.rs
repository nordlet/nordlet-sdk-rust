pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUpdateCatalogRequestComponentsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl ItemsUpdateCatalogRequestComponentsItem {
    pub fn builder() -> ItemsUpdateCatalogRequestComponentsItemBuilder {
        <ItemsUpdateCatalogRequestComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUpdateCatalogRequestComponentsItemBuilder {
    item_id: Option<String>,
    quantity: Option<String>,
}

impl ItemsUpdateCatalogRequestComponentsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUpdateCatalogRequestComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](ItemsUpdateCatalogRequestComponentsItemBuilder::item_id)
    /// - [`quantity`](ItemsUpdateCatalogRequestComponentsItemBuilder::quantity)
    pub fn build(self) -> Result<ItemsUpdateCatalogRequestComponentsItem, BuildError> {
        Ok(ItemsUpdateCatalogRequestComponentsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
