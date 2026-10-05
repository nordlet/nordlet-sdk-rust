pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUpdateCatalogResponseComponentsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub quantity: String,
}

impl ItemsUpdateCatalogResponseComponentsItem {
    pub fn builder() -> ItemsUpdateCatalogResponseComponentsItemBuilder {
        <ItemsUpdateCatalogResponseComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUpdateCatalogResponseComponentsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    quantity: Option<String>,
}

impl ItemsUpdateCatalogResponseComponentsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUpdateCatalogResponseComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](ItemsUpdateCatalogResponseComponentsItemBuilder::item_id)
    /// - [`item_name`](ItemsUpdateCatalogResponseComponentsItemBuilder::item_name)
    /// - [`quantity`](ItemsUpdateCatalogResponseComponentsItemBuilder::quantity)
    pub fn build(self) -> Result<ItemsUpdateCatalogResponseComponentsItem, BuildError> {
        Ok(ItemsUpdateCatalogResponseComponentsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
