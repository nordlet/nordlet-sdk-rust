pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsUpdateCatalogRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<String>,
    #[serde(rename = "isDefaultSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_sale: Option<bool>,
    #[serde(rename = "isDefaultPurchase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_purchase: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
}

impl ItemsUnitsUpdateCatalogRequest {
    pub fn builder() -> ItemsUnitsUpdateCatalogRequestBuilder {
        <ItemsUnitsUpdateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsUpdateCatalogRequestBuilder {
    id: Option<String>,
    factor: Option<String>,
    is_default_sale: Option<bool>,
    is_default_purchase: Option<bool>,
    barcode: Option<String>,
}

impl ItemsUnitsUpdateCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ItemsUnitsUpdateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsUnitsUpdateCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemsUnitsUpdateCatalogRequest, BuildError> {
        Ok(ItemsUnitsUpdateCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            factor: self.factor,
            is_default_sale: self.is_default_sale,
            is_default_purchase: self.is_default_purchase,
            barcode: self.barcode,
        })
    }
}
