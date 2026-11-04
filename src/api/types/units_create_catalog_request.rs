pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsCreateCatalogRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "rec20Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec20_code: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UnitsCreateCatalogRequest {
    pub fn builder() -> UnitsCreateCatalogRequestBuilder {
        <UnitsCreateCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsCreateCatalogRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    rec20_code: Option<String>,
    is_active: Option<bool>,
}

impl UnitsCreateCatalogRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rec20_code(mut self, value: impl Into<String>) -> Self {
        self.rec20_code = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsCreateCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UnitsCreateCatalogRequestBuilder::code)
    /// - [`name`](UnitsCreateCatalogRequestBuilder::name)
    pub fn build(self) -> Result<UnitsCreateCatalogRequest, BuildError> {
        Ok(UnitsCreateCatalogRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rec20_code: self.rec20_code,
            is_active: self.is_active,
        })
    }
}
