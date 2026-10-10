pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WarehousesListInventoryResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl WarehousesListInventoryResponseRowsItem {
    pub fn builder() -> WarehousesListInventoryResponseRowsItemBuilder {
        <WarehousesListInventoryResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarehousesListInventoryResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_default: Option<bool>,
    country_code: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl WarehousesListInventoryResponseRowsItemBuilder {
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

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WarehousesListInventoryResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WarehousesListInventoryResponseRowsItemBuilder::id)
    /// - [`code`](WarehousesListInventoryResponseRowsItemBuilder::code)
    /// - [`name`](WarehousesListInventoryResponseRowsItemBuilder::name)
    /// - [`is_default`](WarehousesListInventoryResponseRowsItemBuilder::is_default)
    /// - [`created_at`](WarehousesListInventoryResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<WarehousesListInventoryResponseRowsItem, BuildError> {
        Ok(WarehousesListInventoryResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            country_code: self.country_code,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
