pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsGetResponseComponentsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub quantity: String,
}

impl PostV1CatalogItemsGetResponseComponentsItem {
    pub fn builder() -> PostV1CatalogItemsGetResponseComponentsItemBuilder {
        <PostV1CatalogItemsGetResponseComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsGetResponseComponentsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    quantity: Option<String>,
}

impl PostV1CatalogItemsGetResponseComponentsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1CatalogItemsGetResponseComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1CatalogItemsGetResponseComponentsItemBuilder::item_id)
    /// - [`item_name`](PostV1CatalogItemsGetResponseComponentsItemBuilder::item_name)
    /// - [`quantity`](PostV1CatalogItemsGetResponseComponentsItemBuilder::quantity)
    pub fn build(self) -> Result<PostV1CatalogItemsGetResponseComponentsItem, BuildError> {
        Ok(PostV1CatalogItemsGetResponseComponentsItem {
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
