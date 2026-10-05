pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsSetCatalogRequest {
    #[serde(rename = "priceListId")]
    #[serde(default)]
    pub price_list_id: String,
    #[serde(default)]
    pub items: Vec<PriceListsItemsSetCatalogRequestItemsItem>,
}

impl PriceListsItemsSetCatalogRequest {
    pub fn builder() -> PriceListsItemsSetCatalogRequestBuilder {
        <PriceListsItemsSetCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsSetCatalogRequestBuilder {
    price_list_id: Option<String>,
    items: Option<Vec<PriceListsItemsSetCatalogRequestItemsItem>>,
}

impl PriceListsItemsSetCatalogRequestBuilder {
    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<PriceListsItemsSetCatalogRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsSetCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_list_id`](PriceListsItemsSetCatalogRequestBuilder::price_list_id)
    /// - [`items`](PriceListsItemsSetCatalogRequestBuilder::items)
    pub fn build(self) -> Result<PriceListsItemsSetCatalogRequest, BuildError> {
        Ok(PriceListsItemsSetCatalogRequest {
            price_list_id: self
                .price_list_id
                .ok_or_else(|| BuildError::missing_field("price_list_id"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
