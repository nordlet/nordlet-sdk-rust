pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsUpdateCatalogResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl PriceListsUpdateCatalogResponse {
    pub fn builder() -> PriceListsUpdateCatalogResponseBuilder {
        <PriceListsUpdateCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsUpdateCatalogResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    is_active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl PriceListsUpdateCatalogResponseBuilder {
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceListsUpdateCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PriceListsUpdateCatalogResponseBuilder::id)
    /// - [`code`](PriceListsUpdateCatalogResponseBuilder::code)
    /// - [`name`](PriceListsUpdateCatalogResponseBuilder::name)
    /// - [`currency`](PriceListsUpdateCatalogResponseBuilder::currency)
    /// - [`is_active`](PriceListsUpdateCatalogResponseBuilder::is_active)
    /// - [`created_at`](PriceListsUpdateCatalogResponseBuilder::created_at)
    pub fn build(self) -> Result<PriceListsUpdateCatalogResponse, BuildError> {
        Ok(PriceListsUpdateCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
