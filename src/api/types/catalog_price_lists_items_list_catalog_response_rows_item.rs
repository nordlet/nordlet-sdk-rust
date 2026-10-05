pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsListCatalogResponseRowsItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
    #[serde(rename = "itemCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_code: Option<String>,
    #[serde(rename = "unitPriceExclVat")]
    #[serde(default)]
    pub unit_price_excl_vat: String,
}

impl PriceListsItemsListCatalogResponseRowsItem {
    pub fn builder() -> PriceListsItemsListCatalogResponseRowsItemBuilder {
        <PriceListsItemsListCatalogResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsListCatalogResponseRowsItemBuilder {
    item_id: Option<String>,
    item_name: Option<String>,
    item_code: Option<String>,
    unit_price_excl_vat: Option<String>,
}

impl PriceListsItemsListCatalogResponseRowsItemBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn item_name(mut self, value: impl Into<String>) -> Self {
        self.item_name = Some(value.into());
        self
    }

    pub fn item_code(mut self, value: impl Into<String>) -> Self {
        self.item_code = Some(value.into());
        self
    }

    pub fn unit_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.unit_price_excl_vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsListCatalogResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PriceListsItemsListCatalogResponseRowsItemBuilder::item_id)
    /// - [`item_name`](PriceListsItemsListCatalogResponseRowsItemBuilder::item_name)
    /// - [`unit_price_excl_vat`](PriceListsItemsListCatalogResponseRowsItemBuilder::unit_price_excl_vat)
    pub fn build(self) -> Result<PriceListsItemsListCatalogResponseRowsItem, BuildError> {
        Ok(PriceListsItemsListCatalogResponseRowsItem {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            item_name: self
                .item_name
                .ok_or_else(|| BuildError::missing_field("item_name"))?,
            item_code: self.item_code,
            unit_price_excl_vat: self
                .unit_price_excl_vat
                .ok_or_else(|| BuildError::missing_field("unit_price_excl_vat"))?,
        })
    }
}
