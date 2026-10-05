pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItemsKindsListCatalogResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "saftType")]
    pub saft_type: ItemsKindsListCatalogResponseRowsItemSaftType,
    #[serde(rename = "quantityAccounting")]
    #[serde(default)]
    pub quantity_accounting: bool,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ItemsKindsListCatalogResponseRowsItem {
    pub fn builder() -> ItemsKindsListCatalogResponseRowsItemBuilder {
        <ItemsKindsListCatalogResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsListCatalogResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    saft_type: Option<ItemsKindsListCatalogResponseRowsItemSaftType>,
    quantity_accounting: Option<bool>,
    sort_order: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ItemsKindsListCatalogResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn saft_type(mut self, value: ItemsKindsListCatalogResponseRowsItemSaftType) -> Self {
        self.saft_type = Some(value);
        self
    }

    pub fn quantity_accounting(mut self, value: bool) -> Self {
        self.quantity_accounting = Some(value);
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsKindsListCatalogResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsKindsListCatalogResponseRowsItemBuilder::id)
    /// - [`code`](ItemsKindsListCatalogResponseRowsItemBuilder::code)
    /// - [`name`](ItemsKindsListCatalogResponseRowsItemBuilder::name)
    /// - [`saft_type`](ItemsKindsListCatalogResponseRowsItemBuilder::saft_type)
    /// - [`quantity_accounting`](ItemsKindsListCatalogResponseRowsItemBuilder::quantity_accounting)
    /// - [`sort_order`](ItemsKindsListCatalogResponseRowsItemBuilder::sort_order)
    /// - [`created_at`](ItemsKindsListCatalogResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<ItemsKindsListCatalogResponseRowsItem, BuildError> {
        Ok(ItemsKindsListCatalogResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            saft_type: self
                .saft_type
                .ok_or_else(|| BuildError::missing_field("saft_type"))?,
            quantity_accounting: self
                .quantity_accounting
                .ok_or_else(|| BuildError::missing_field("quantity_accounting"))?,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
