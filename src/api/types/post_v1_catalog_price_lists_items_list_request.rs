pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsItemsListRequest {
    #[serde(rename = "priceListId")]
    #[serde(default)]
    pub price_list_id: String,
}

impl PostV1CatalogPriceListsItemsListRequest {
    pub fn builder() -> PostV1CatalogPriceListsItemsListRequestBuilder {
        <PostV1CatalogPriceListsItemsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsItemsListRequestBuilder {
    price_list_id: Option<String>,
}

impl PostV1CatalogPriceListsItemsListRequestBuilder {
    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsItemsListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_list_id`](PostV1CatalogPriceListsItemsListRequestBuilder::price_list_id)
    pub fn build(self) -> Result<PostV1CatalogPriceListsItemsListRequest, BuildError> {
        Ok(PostV1CatalogPriceListsItemsListRequest {
            price_list_id: self
                .price_list_id
                .ok_or_else(|| BuildError::missing_field("price_list_id"))?,
        })
    }
}
