pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1CatalogUnitsCreateResponse {
    pub fn builder() -> PostV1CatalogUnitsCreateResponseBuilder {
        <PostV1CatalogUnitsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1CatalogUnitsCreateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogUnitsCreateResponseBuilder::id)
    /// - [`code`](PostV1CatalogUnitsCreateResponseBuilder::code)
    /// - [`name`](PostV1CatalogUnitsCreateResponseBuilder::name)
    /// - [`is_active`](PostV1CatalogUnitsCreateResponseBuilder::is_active)
    /// - [`created_at`](PostV1CatalogUnitsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1CatalogUnitsCreateResponse, BuildError> {
        Ok(PostV1CatalogUnitsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
