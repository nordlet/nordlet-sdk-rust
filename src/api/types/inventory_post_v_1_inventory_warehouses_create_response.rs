pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryWarehousesCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1InventoryWarehousesCreateResponse {
    pub fn builder() -> PostV1InventoryWarehousesCreateResponseBuilder {
        <PostV1InventoryWarehousesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryWarehousesCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_default: Option<bool>,
    created_at: Option<String>,
}

impl PostV1InventoryWarehousesCreateResponseBuilder {
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

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryWarehousesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryWarehousesCreateResponseBuilder::id)
    /// - [`code`](PostV1InventoryWarehousesCreateResponseBuilder::code)
    /// - [`name`](PostV1InventoryWarehousesCreateResponseBuilder::name)
    /// - [`is_default`](PostV1InventoryWarehousesCreateResponseBuilder::is_default)
    /// - [`created_at`](PostV1InventoryWarehousesCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1InventoryWarehousesCreateResponse, BuildError> {
        Ok(PostV1InventoryWarehousesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
