pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WarehousesCreateInventoryRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl WarehousesCreateInventoryRequest {
    pub fn builder() -> WarehousesCreateInventoryRequestBuilder {
        <WarehousesCreateInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarehousesCreateInventoryRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    is_default: Option<bool>,
}

impl WarehousesCreateInventoryRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WarehousesCreateInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](WarehousesCreateInventoryRequestBuilder::code)
    /// - [`name`](WarehousesCreateInventoryRequestBuilder::name)
    pub fn build(self) -> Result<WarehousesCreateInventoryRequest, BuildError> {
        Ok(WarehousesCreateInventoryRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_default: self.is_default,
        })
    }
}
