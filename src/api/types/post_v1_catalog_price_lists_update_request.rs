pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl PostV1CatalogPriceListsUpdateRequest {
    pub fn builder() -> PostV1CatalogPriceListsUpdateRequestBuilder {
        <PostV1CatalogPriceListsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    currency: Option<String>,
    is_active: Option<bool>,
}

impl PostV1CatalogPriceListsUpdateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogPriceListsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogPriceListsUpdateRequest, BuildError> {
        Ok(PostV1CatalogPriceListsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            currency: self.currency,
            is_active: self.is_active,
        })
    }
}
