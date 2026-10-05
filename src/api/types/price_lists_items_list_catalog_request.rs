pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsListCatalogRequest {
    #[serde(rename = "priceListId")]
    #[serde(default)]
    pub price_list_id: String,
}

impl PriceListsItemsListCatalogRequest {
    pub fn builder() -> PriceListsItemsListCatalogRequestBuilder {
        <PriceListsItemsListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsListCatalogRequestBuilder {
    price_list_id: Option<String>,
}

impl PriceListsItemsListCatalogRequestBuilder {
    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsListCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_list_id`](PriceListsItemsListCatalogRequestBuilder::price_list_id)
    pub fn build(self) -> Result<PriceListsItemsListCatalogRequest, BuildError> {
        Ok(PriceListsItemsListCatalogRequest {
            price_list_id: self
                .price_list_id
                .ok_or_else(|| BuildError::missing_field("price_list_id"))?,
        })
    }
}
