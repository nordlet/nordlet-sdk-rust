pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryWarehousesListResponseRowsItem {
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

impl PostV1InventoryWarehousesListResponseRowsItem {
    pub fn builder() -> PostV1InventoryWarehousesListResponseRowsItemBuilder {
        <PostV1InventoryWarehousesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryWarehousesListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_default: Option<bool>,
    created_at: Option<String>,
}

impl PostV1InventoryWarehousesListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1InventoryWarehousesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryWarehousesListResponseRowsItemBuilder::id)
    /// - [`code`](PostV1InventoryWarehousesListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1InventoryWarehousesListResponseRowsItemBuilder::name)
    /// - [`is_default`](PostV1InventoryWarehousesListResponseRowsItemBuilder::is_default)
    /// - [`created_at`](PostV1InventoryWarehousesListResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<PostV1InventoryWarehousesListResponseRowsItem, BuildError> {
        Ok(PostV1InventoryWarehousesListResponseRowsItem {
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
