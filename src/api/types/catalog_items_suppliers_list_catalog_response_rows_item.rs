pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsSuppliersListCatalogResponseRowsItem {
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ItemsSuppliersListCatalogResponseRowsItem {
    pub fn builder() -> ItemsSuppliersListCatalogResponseRowsItemBuilder {
        <ItemsSuppliersListCatalogResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsSuppliersListCatalogResponseRowsItemBuilder {
    id: Option<String>,
    item_id: Option<String>,
    partner_id: Option<String>,
    partner_name: Option<String>,
    supplier_code: Option<String>,
    purchase_price_excl_vat: Option<String>,
    currency: Option<String>,
    notes: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ItemsSuppliersListCatalogResponseRowsItemBuilder {
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsSuppliersListCatalogResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsSuppliersListCatalogResponseRowsItemBuilder::id)
    /// - [`item_id`](ItemsSuppliersListCatalogResponseRowsItemBuilder::item_id)
    /// - [`partner_id`](ItemsSuppliersListCatalogResponseRowsItemBuilder::partner_id)
    /// - [`partner_name`](ItemsSuppliersListCatalogResponseRowsItemBuilder::partner_name)
    /// - [`currency`](ItemsSuppliersListCatalogResponseRowsItemBuilder::currency)
    /// - [`updated_at`](ItemsSuppliersListCatalogResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<ItemsSuppliersListCatalogResponseRowsItem, BuildError> {
        Ok(ItemsSuppliersListCatalogResponseRowsItem {
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
