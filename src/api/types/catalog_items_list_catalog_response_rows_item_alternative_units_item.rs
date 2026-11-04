pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsListCatalogResponseRowsItemAlternativeUnitsItem {
    #[serde(rename = "unitCode")]
    #[serde(default)]
    pub unit_code: String,
    #[serde(default)]
    pub factor: String,
    #[serde(rename = "isDefaultSale")]
    #[serde(default)]
    pub is_default_sale: bool,
    #[serde(rename = "isDefaultPurchase")]
    #[serde(default)]
    pub is_default_purchase: bool,
    #[serde(rename = "salePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price_excl_vat: Option<String>,
    #[serde(rename = "purchasePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price_excl_vat: Option<String>,
}

impl ItemsListCatalogResponseRowsItemAlternativeUnitsItem {
    pub fn builder() -> ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder {
        <ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder {
    unit_code: Option<String>,
    factor: Option<String>,
    is_default_sale: Option<bool>,
    is_default_purchase: Option<bool>,
    sale_price_excl_vat: Option<String>,
    purchase_price_excl_vat: Option<String>,
}

impl ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder {
    pub fn unit_code(mut self, value: impl Into<String>) -> Self {
        self.unit_code = Some(value.into());
        self
    }

    pub fn factor(mut self, value: impl Into<String>) -> Self {
        self.factor = Some(value.into());
        self
    }

    pub fn is_default_sale(mut self, value: bool) -> Self {
        self.is_default_sale = Some(value);
        self
    }

    pub fn is_default_purchase(mut self, value: bool) -> Self {
        self.is_default_purchase = Some(value);
        self
    }

    pub fn sale_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.sale_price_excl_vat = Some(value.into());
        self
    }

    pub fn purchase_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.purchase_price_excl_vat = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsListCatalogResponseRowsItemAlternativeUnitsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit_code`](ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder::unit_code)
    /// - [`factor`](ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder::factor)
    /// - [`is_default_sale`](ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder::is_default_sale)
    /// - [`is_default_purchase`](ItemsListCatalogResponseRowsItemAlternativeUnitsItemBuilder::is_default_purchase)
    pub fn build(self) -> Result<ItemsListCatalogResponseRowsItemAlternativeUnitsItem, BuildError> {
        Ok(ItemsListCatalogResponseRowsItemAlternativeUnitsItem {
            unit_code: self
                .unit_code
                .ok_or_else(|| BuildError::missing_field("unit_code"))?,
            factor: self
                .factor
                .ok_or_else(|| BuildError::missing_field("factor"))?,
            is_default_sale: self
                .is_default_sale
                .ok_or_else(|| BuildError::missing_field("is_default_sale"))?,
            is_default_purchase: self
                .is_default_purchase
                .ok_or_else(|| BuildError::missing_field("is_default_purchase"))?,
            sale_price_excl_vat: self.sale_price_excl_vat,
            purchase_price_excl_vat: self.purchase_price_excl_vat,
        })
    }
}
