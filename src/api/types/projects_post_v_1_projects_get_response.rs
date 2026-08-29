pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "partnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    pub status: PostV1ProjectsGetResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1ProjectsGetResponse {
    pub fn builder() -> PostV1ProjectsGetResponseBuilder {
        <PostV1ProjectsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsGetResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    partner_id: Option<String>,
    status: Option<PostV1ProjectsGetResponseStatus>,
    notes: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1ProjectsGetResponseBuilder {
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1ProjectsGetResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsGetResponseBuilder::id)
    /// - [`code`](PostV1ProjectsGetResponseBuilder::code)
    /// - [`name`](PostV1ProjectsGetResponseBuilder::name)
    /// - [`status`](PostV1ProjectsGetResponseBuilder::status)
    /// - [`created_at`](PostV1ProjectsGetResponseBuilder::created_at)
    /// - [`updated_at`](PostV1ProjectsGetResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1ProjectsGetResponse, BuildError> {
        Ok(PostV1ProjectsGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            partner_id: self.partner_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
