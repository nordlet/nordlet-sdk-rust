pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsUpdateResponse {
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
    pub created_at: String,
}

impl PostV1CatalogPriceListsUpdateResponse {
    pub fn builder() -> PostV1CatalogPriceListsUpdateResponseBuilder {
        <PostV1CatalogPriceListsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsUpdateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1CatalogPriceListsUpdateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogPriceListsUpdateResponseBuilder::id)
    /// - [`code`](PostV1CatalogPriceListsUpdateResponseBuilder::code)
    /// - [`name`](PostV1CatalogPriceListsUpdateResponseBuilder::name)
    /// - [`currency`](PostV1CatalogPriceListsUpdateResponseBuilder::currency)
    /// - [`is_active`](PostV1CatalogPriceListsUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1CatalogPriceListsUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1CatalogPriceListsUpdateResponse, BuildError> {
        Ok(PostV1CatalogPriceListsUpdateResponse {
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
