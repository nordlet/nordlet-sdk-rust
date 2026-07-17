pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsItemsDeleteRequest {
    #[serde(rename = "priceListId")]
    #[serde(default)]
    pub price_list_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
}

impl PostV1CatalogPriceListsItemsDeleteRequest {
    pub fn builder() -> PostV1CatalogPriceListsItemsDeleteRequestBuilder {
        <PostV1CatalogPriceListsItemsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsItemsDeleteRequestBuilder {
    price_list_id: Option<String>,
    item_id: Option<String>,
}

impl PostV1CatalogPriceListsItemsDeleteRequestBuilder {
    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsItemsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_list_id`](PostV1CatalogPriceListsItemsDeleteRequestBuilder::price_list_id)
    /// - [`item_id`](PostV1CatalogPriceListsItemsDeleteRequestBuilder::item_id)
    pub fn build(self) -> Result<PostV1CatalogPriceListsItemsDeleteRequest, BuildError> {
        Ok(PostV1CatalogPriceListsItemsDeleteRequest {
            price_list_id: self
                .price_list_id
                .ok_or_else(|| BuildError::missing_field("price_list_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
        })
    }
}
