pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WarehousesUpdateInventoryRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

impl WarehousesUpdateInventoryRequest {
    pub fn builder() -> WarehousesUpdateInventoryRequestBuilder {
        <WarehousesUpdateInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarehousesUpdateInventoryRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    country_code: Option<String>,
}

impl WarehousesUpdateInventoryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WarehousesUpdateInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WarehousesUpdateInventoryRequestBuilder::id)
    pub fn build(self) -> Result<WarehousesUpdateInventoryRequest, BuildError> {
        Ok(WarehousesUpdateInventoryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            country_code: self.country_code,
        })
    }
}
