pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsUpdateCatalogRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UnitsUpdateCatalogRequest {
    pub fn builder() -> UnitsUpdateCatalogRequestBuilder {
        <UnitsUpdateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsUpdateCatalogRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
}

impl UnitsUpdateCatalogRequestBuilder {
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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsUpdateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UnitsUpdateCatalogRequestBuilder::id)
    pub fn build(self) -> Result<UnitsUpdateCatalogRequest, BuildError> {
        Ok(UnitsUpdateCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            is_active: self.is_active,
        })
    }
}
