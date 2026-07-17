pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsCreateRequest {
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

impl PostV1CatalogPriceListsCreateRequest {
    pub fn builder() -> PostV1CatalogPriceListsCreateRequestBuilder {
        <PostV1CatalogPriceListsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    is_active: Option<bool>,
}

impl PostV1CatalogPriceListsCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1CatalogPriceListsCreateRequestBuilder::code)
    /// - [`name`](PostV1CatalogPriceListsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1CatalogPriceListsCreateRequest, BuildError> {
        Ok(PostV1CatalogPriceListsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            currency: self.currency,
            is_active: self.is_active,
        })
    }
}
