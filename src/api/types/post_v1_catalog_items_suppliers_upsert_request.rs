pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsSuppliersUpsertRequest {
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

impl PostV1CatalogItemsSuppliersUpsertRequest {
    pub fn builder() -> PostV1CatalogItemsSuppliersUpsertRequestBuilder {
        <PostV1CatalogItemsSuppliersUpsertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsSuppliersUpsertRequestBuilder {
    item_id: Option<String>,
    partner_id: Option<String>,
    supplier_code: Option<String>,
    purchase_price_excl_vat: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
}

impl PostV1CatalogItemsSuppliersUpsertRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1CatalogItemsSuppliersUpsertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1CatalogItemsSuppliersUpsertRequestBuilder::item_id)
    /// - [`partner_id`](PostV1CatalogItemsSuppliersUpsertRequestBuilder::partner_id)
    pub fn build(self) -> Result<PostV1CatalogItemsSuppliersUpsertRequest, BuildError> {
        Ok(PostV1CatalogItemsSuppliersUpsertRequest {
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
