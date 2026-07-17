pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemGroupsCreateResponse {
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
    pub created_at: String,
}

impl PostV1CatalogItemGroupsCreateResponse {
    pub fn builder() -> PostV1CatalogItemGroupsCreateResponseBuilder {
        <PostV1CatalogItemGroupsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemGroupsCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    parent_id: Option<String>,
    created_at: Option<String>,
}

impl PostV1CatalogItemGroupsCreateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemGroupsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemGroupsCreateResponseBuilder::id)
    /// - [`code`](PostV1CatalogItemGroupsCreateResponseBuilder::code)
    /// - [`name`](PostV1CatalogItemGroupsCreateResponseBuilder::name)
    /// - [`created_at`](PostV1CatalogItemGroupsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1CatalogItemGroupsCreateResponse, BuildError> {
        Ok(PostV1CatalogItemGroupsCreateResponse {
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
