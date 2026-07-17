pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsItemsSetRequest {
    #[serde(rename = "priceListId")]
    #[serde(default)]
    pub price_list_id: String,
    #[serde(default)]
    pub items: Vec<PostV1CatalogPriceListsItemsSetRequestItemsItem>,
}

impl PostV1CatalogPriceListsItemsSetRequest {
    pub fn builder() -> PostV1CatalogPriceListsItemsSetRequestBuilder {
        <PostV1CatalogPriceListsItemsSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsItemsSetRequestBuilder {
    price_list_id: Option<String>,
    items: Option<Vec<PostV1CatalogPriceListsItemsSetRequestItemsItem>>,
}

impl PostV1CatalogPriceListsItemsSetRequestBuilder {
    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<PostV1CatalogPriceListsItemsSetRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsItemsSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_list_id`](PostV1CatalogPriceListsItemsSetRequestBuilder::price_list_id)
    /// - [`items`](PostV1CatalogPriceListsItemsSetRequestBuilder::items)
    pub fn build(self) -> Result<PostV1CatalogPriceListsItemsSetRequest, BuildError> {
        Ok(PostV1CatalogPriceListsItemsSetRequest {
            price_list_id: self
                .price_list_id
                .ok_or_else(|| BuildError::missing_field("price_list_id"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
