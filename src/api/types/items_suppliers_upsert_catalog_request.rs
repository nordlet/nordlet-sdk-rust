pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsSuppliersUpsertCatalogRequest {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "supplierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_code: Option<String>,
    #[serde(rename = "purchasePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price_excl_vat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ItemsSuppliersUpsertCatalogRequest {
    pub fn builder() -> ItemsSuppliersUpsertCatalogRequestBuilder {
        <ItemsSuppliersUpsertCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsSuppliersUpsertCatalogRequestBuilder {
    item_id: Option<String>,
    partner_id: Option<String>,
    supplier_code: Option<String>,
    purchase_price_excl_vat: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
}

impl ItemsSuppliersUpsertCatalogRequestBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn supplier_code(mut self, value: impl Into<String>) -> Self {
        self.supplier_code = Some(value.into());
        self
    }

    pub fn purchase_price_excl_vat(mut self, value: impl Into<String>) -> Self {
        self.purchase_price_excl_vat = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsSuppliersUpsertCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](ItemsSuppliersUpsertCatalogRequestBuilder::item_id)
    /// - [`partner_id`](ItemsSuppliersUpsertCatalogRequestBuilder::partner_id)
    pub fn build(self) -> Result<ItemsSuppliersUpsertCatalogRequest, BuildError> {
        Ok(ItemsSuppliersUpsertCatalogRequest {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            supplier_code: self.supplier_code,
            purchase_price_excl_vat: self.purchase_price_excl_vat,
            currency: self.currency,
            notes: self.notes,
        })
    }
}
