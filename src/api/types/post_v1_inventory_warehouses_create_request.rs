pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryWarehousesCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl PostV1InventoryWarehousesCreateRequest {
    pub fn builder() -> PostV1InventoryWarehousesCreateRequestBuilder {
        <PostV1InventoryWarehousesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryWarehousesCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    is_default: Option<bool>,
}

impl PostV1InventoryWarehousesCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1InventoryWarehousesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1InventoryWarehousesCreateRequestBuilder::code)
    /// - [`name`](PostV1InventoryWarehousesCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1InventoryWarehousesCreateRequest, BuildError> {
        Ok(PostV1InventoryWarehousesCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_default: self.is_default,
        })
    }
}
