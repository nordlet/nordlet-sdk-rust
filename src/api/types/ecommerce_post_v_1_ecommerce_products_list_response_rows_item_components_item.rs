pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceProductsListResponseRowsItemComponentsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub quantity: String,
}

impl PostV1EcommerceProductsListResponseRowsItemComponentsItem {
    pub fn builder() -> PostV1EcommerceProductsListResponseRowsItemComponentsItemBuilder {
        <PostV1EcommerceProductsListResponseRowsItemComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceProductsListResponseRowsItemComponentsItemBuilder {
    item_id: Option<String>,
    quantity: Option<String>,
}

impl PostV1EcommerceProductsListResponseRowsItemComponentsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceProductsListResponseRowsItemComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1EcommerceProductsListResponseRowsItemComponentsItemBuilder::item_id)
    /// - [`quantity`](PostV1EcommerceProductsListResponseRowsItemComponentsItemBuilder::quantity)
    pub fn build(
        self,
    ) -> Result<PostV1EcommerceProductsListResponseRowsItemComponentsItem, BuildError> {
        Ok(PostV1EcommerceProductsListResponseRowsItemComponentsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
