pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsDeleteCatalogRequest {
    #[serde(rename = "priceListId")]
    #[serde(default)]
    pub price_list_id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
}

impl PriceListsItemsDeleteCatalogRequest {
    pub fn builder() -> PriceListsItemsDeleteCatalogRequestBuilder {
        <PriceListsItemsDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsDeleteCatalogRequestBuilder {
    price_list_id: Option<String>,
    item_id: Option<String>,
}

impl PriceListsItemsDeleteCatalogRequestBuilder {
    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_list_id`](PriceListsItemsDeleteCatalogRequestBuilder::price_list_id)
    /// - [`item_id`](PriceListsItemsDeleteCatalogRequestBuilder::item_id)
    pub fn build(self) -> Result<PriceListsItemsDeleteCatalogRequest, BuildError> {
        Ok(PriceListsItemsDeleteCatalogRequest {
            price_list_id: self
                .price_list_id
                .ok_or_else(|| BuildError::missing_field("price_list_id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
        })
    }
}
