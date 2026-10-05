pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemGroupsUpdateCatalogResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ItemGroupsUpdateCatalogResponse {
    pub fn builder() -> ItemGroupsUpdateCatalogResponseBuilder {
        <ItemGroupsUpdateCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemGroupsUpdateCatalogResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    parent_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ItemGroupsUpdateCatalogResponseBuilder {
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

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemGroupsUpdateCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemGroupsUpdateCatalogResponseBuilder::id)
    /// - [`code`](ItemGroupsUpdateCatalogResponseBuilder::code)
    /// - [`name`](ItemGroupsUpdateCatalogResponseBuilder::name)
    /// - [`created_at`](ItemGroupsUpdateCatalogResponseBuilder::created_at)
    pub fn build(self) -> Result<ItemGroupsUpdateCatalogResponse, BuildError> {
        Ok(ItemGroupsUpdateCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_id: self.parent_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
