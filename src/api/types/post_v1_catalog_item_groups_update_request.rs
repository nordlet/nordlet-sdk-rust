pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemGroupsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl PostV1CatalogItemGroupsUpdateRequest {
    pub fn builder() -> PostV1CatalogItemGroupsUpdateRequestBuilder {
        <PostV1CatalogItemGroupsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemGroupsUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    parent_id: Option<String>,
}

impl PostV1CatalogItemGroupsUpdateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1CatalogItemGroupsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemGroupsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemGroupsUpdateRequest, BuildError> {
        Ok(PostV1CatalogItemGroupsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            parent_id: self.parent_id,
        })
    }
}
