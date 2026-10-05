pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsSetCatalogRequestItemsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(default)]
    pub unit_price_excl_vat: String,
}

impl PriceListsItemsSetCatalogRequestItemsItem {
    pub fn builder() -> PriceListsItemsSetCatalogRequestItemsItemBuilder {
        <PriceListsItemsSetCatalogRequestItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsSetCatalogRequestItemsItemBuilder {
    item_id: Option<String>,
    unit_price_excl_vat: Option<String>,
}

impl PriceListsItemsSetCatalogRequestItemsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsSetCatalogRequestItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PriceListsItemsSetCatalogRequestItemsItemBuilder::item_id)
    /// - [`unit_price_excl_vat`](PriceListsItemsSetCatalogRequestItemsItemBuilder::unit_price_excl_vat)
    pub fn build(self) -> Result<PriceListsItemsSetCatalogRequestItemsItem, BuildError> {
        Ok(PriceListsItemsSetCatalogRequestItemsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            unit_price_excl_vat: self
                .unit_price_excl_vat
                .ok_or_else(|| BuildError::missing_field("unit_price_excl_vat"))?,
        })
    }
}
