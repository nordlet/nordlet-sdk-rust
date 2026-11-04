pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsCreateCatalogRequest {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "unitCode")]
    #[serde(default)]
    pub unit_code: String,
    #[serde(default)]
    pub factor: String,
    #[serde(rename = "isDefaultSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_sale: Option<bool>,
    #[serde(rename = "isDefaultPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
}

impl ItemsUnitsCreateCatalogRequest {
    pub fn builder() -> ItemsUnitsCreateCatalogRequestBuilder {
        <ItemsUnitsCreateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsCreateCatalogRequestBuilder {
    item_id: Option<String>,
    unit_code: Option<String>,
    factor: Option<String>,
    is_default_sale: Option<bool>,
    is_default_purchase: Option<bool>,
    barcode: Option<String>,
}

impl ItemsUnitsCreateCatalogRequestBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

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

    pub fn barcode(mut self, value: impl Into<String>) -> Self {
        self.barcode = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUnitsCreateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](ItemsUnitsCreateCatalogRequestBuilder::item_id)
    /// - [`unit_code`](ItemsUnitsCreateCatalogRequestBuilder::unit_code)
    /// - [`factor`](ItemsUnitsCreateCatalogRequestBuilder::factor)
    pub fn build(self) -> Result<ItemsUnitsCreateCatalogRequest, BuildError> {
        Ok(ItemsUnitsCreateCatalogRequest {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            unit_code: self
                .unit_code
                .ok_or_else(|| BuildError::missing_field("unit_code"))?,
            factor: self
                .factor
                .ok_or_else(|| BuildError::missing_field("factor"))?,
            is_default_sale: self.is_default_sale,
            is_default_purchase: self.is_default_purchase,
            barcode: self.barcode,
        })
    }
}
