pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItemsKindsUpdateCatalogResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "saftType")]
    pub saft_type: ItemsKindsUpdateCatalogResponseSaftType,
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

impl ItemsKindsUpdateCatalogResponse {
    pub fn builder() -> ItemsKindsUpdateCatalogResponseBuilder {
        <ItemsKindsUpdateCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsUpdateCatalogResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    saft_type: Option<ItemsKindsUpdateCatalogResponseSaftType>,
    quantity_accounting: Option<bool>,
    sort_order: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ItemsKindsUpdateCatalogResponseBuilder {
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

    pub fn saft_type(mut self, value: ItemsKindsUpdateCatalogResponseSaftType) -> Self {
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

    /// Consumes the builder and constructs a [`ItemsKindsUpdateCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsKindsUpdateCatalogResponseBuilder::id)
    /// - [`code`](ItemsKindsUpdateCatalogResponseBuilder::code)
    /// - [`name`](ItemsKindsUpdateCatalogResponseBuilder::name)
    /// - [`saft_type`](ItemsKindsUpdateCatalogResponseBuilder::saft_type)
    /// - [`quantity_accounting`](ItemsKindsUpdateCatalogResponseBuilder::quantity_accounting)
    /// - [`sort_order`](ItemsKindsUpdateCatalogResponseBuilder::sort_order)
    /// - [`created_at`](ItemsKindsUpdateCatalogResponseBuilder::created_at)
    pub fn build(self) -> Result<ItemsKindsUpdateCatalogResponse, BuildError> {
        Ok(ItemsKindsUpdateCatalogResponse {
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
