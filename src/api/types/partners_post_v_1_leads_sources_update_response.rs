pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1LeadsSourcesUpdateResponse {
    pub fn builder() -> PostV1LeadsSourcesUpdateResponseBuilder {
        <PostV1LeadsSourcesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesUpdateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1LeadsSourcesUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1LeadsSourcesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsSourcesUpdateResponseBuilder::id)
    /// - [`name`](PostV1LeadsSourcesUpdateResponseBuilder::name)
    /// - [`is_active`](PostV1LeadsSourcesUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1LeadsSourcesUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1LeadsSourcesUpdateResponse, BuildError> {
        Ok(PostV1LeadsSourcesUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
