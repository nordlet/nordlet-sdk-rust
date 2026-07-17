pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsSuppliersListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "supplierCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_code: Option<String>,
    #[serde(rename = "purchasePriceExclVat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price_excl_vat: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1CatalogItemsSuppliersListResponseRowsItem {
    pub fn builder() -> PostV1CatalogItemsSuppliersListResponseRowsItemBuilder {
        <PostV1CatalogItemsSuppliersListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsSuppliersListResponseRowsItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    supplier_code: Option<String>,
    purchase_price_excl_vat: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    updated_at: Option<String>,
}

impl PostV1CatalogItemsSuppliersListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsSuppliersListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsSuppliersListResponseRowsItemBuilder::id)
    /// - [`item_id`](PostV1CatalogItemsSuppliersListResponseRowsItemBuilder::item_id)
    /// - [`partner_id`](PostV1CatalogItemsSuppliersListResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](PostV1CatalogItemsSuppliersListResponseRowsItemBuilder::partner_name)
    /// - [`currency`](PostV1CatalogItemsSuppliersListResponseRowsItemBuilder::currency)
    /// - [`updated_at`](PostV1CatalogItemsSuppliersListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1CatalogItemsSuppliersListResponseRowsItem, BuildError> {
        Ok(PostV1CatalogItemsSuppliersListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            supplier_code: self.supplier_code,
            purchase_price_excl_vat: self.purchase_price_excl_vat,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            notes: self.notes,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
