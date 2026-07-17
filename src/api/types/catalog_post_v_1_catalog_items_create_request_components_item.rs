pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsCreateRequestComponentsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl PostV1CatalogItemsCreateRequestComponentsItem {
    pub fn builder() -> PostV1CatalogItemsCreateRequestComponentsItemBuilder {
        <PostV1CatalogItemsCreateRequestComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsCreateRequestComponentsItemBuilder {
    item_id: Option<String>,
    quantity: Option<String>,
}

impl PostV1CatalogItemsCreateRequestComponentsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsCreateRequestComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1CatalogItemsCreateRequestComponentsItemBuilder::item_id)
    /// - [`quantity`](PostV1CatalogItemsCreateRequestComponentsItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1CatalogItemsCreateRequestComponentsItem, BuildError> {
        Ok(PostV1CatalogItemsCreateRequestComponentsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
