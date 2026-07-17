pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsUpdateRequestComponentsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl PostV1CatalogItemsUpdateRequestComponentsItem {
    pub fn builder() -> PostV1CatalogItemsUpdateRequestComponentsItemBuilder {
        <PostV1CatalogItemsUpdateRequestComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsUpdateRequestComponentsItemBuilder {
    item_id: Option<String>,
    quantity: Option<String>,
}

impl PostV1CatalogItemsUpdateRequestComponentsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsUpdateRequestComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1CatalogItemsUpdateRequestComponentsItemBuilder::item_id)
    /// - [`quantity`](PostV1CatalogItemsUpdateRequestComponentsItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1CatalogItemsUpdateRequestComponentsItem, BuildError> {
        Ok(PostV1CatalogItemsUpdateRequestComponentsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
