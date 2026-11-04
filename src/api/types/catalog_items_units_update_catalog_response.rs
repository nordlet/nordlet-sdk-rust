pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsUpdateCatalogResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ItemsUnitsUpdateCatalogResponse {
    pub fn builder() -> ItemsUnitsUpdateCatalogResponseBuilder {
        <ItemsUnitsUpdateCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsUpdateCatalogResponseBuilder {
    id: Option<String>,
    item_id: Option<String>,
    unit_code: Option<String>,
    factor: Option<String>,
    is_default_sale: Option<bool>,
    is_default_purchase: Option<bool>,
    barcode: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ItemsUnitsUpdateCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsUnitsUpdateCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsUnitsUpdateCatalogResponseBuilder::id)
    /// - [`item_id`](ItemsUnitsUpdateCatalogResponseBuilder::item_id)
    /// - [`unit_code`](ItemsUnitsUpdateCatalogResponseBuilder::unit_code)
    /// - [`factor`](ItemsUnitsUpdateCatalogResponseBuilder::factor)
    /// - [`is_default_sale`](ItemsUnitsUpdateCatalogResponseBuilder::is_default_sale)
    /// - [`is_default_purchase`](ItemsUnitsUpdateCatalogResponseBuilder::is_default_purchase)
    /// - [`created_at`](ItemsUnitsUpdateCatalogResponseBuilder::created_at)
    /// - [`updated_at`](ItemsUnitsUpdateCatalogResponseBuilder::updated_at)
    pub fn build(self) -> Result<ItemsUnitsUpdateCatalogResponse, BuildError> {
        Ok(ItemsUnitsUpdateCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
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
            barcode: self.barcode,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
