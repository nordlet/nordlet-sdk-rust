pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsCreateCatalogRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl PriceListsCreateCatalogRequest {
    pub fn builder() -> PriceListsCreateCatalogRequestBuilder {
        <PriceListsCreateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsCreateCatalogRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    is_active: Option<bool>,
}

impl PriceListsCreateCatalogRequestBuilder {
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

    /// Consumes the builder and constructs a [`PriceListsCreateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PriceListsCreateCatalogRequestBuilder::code)
    /// - [`name`](PriceListsCreateCatalogRequestBuilder::name)
    pub fn build(self) -> Result<PriceListsCreateCatalogRequest, BuildError> {
        Ok(PriceListsCreateCatalogRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            currency: self.currency,
            is_active: self.is_active,
        })
    }
}
