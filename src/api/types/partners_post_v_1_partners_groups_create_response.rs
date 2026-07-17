pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGroupsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PartnersGroupsCreateResponse {
    pub fn builder() -> PostV1PartnersGroupsCreateResponseBuilder {
        <PostV1PartnersGroupsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGroupsCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    created_at: Option<String>,
}

impl PostV1PartnersGroupsCreateResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersGroupsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersGroupsCreateResponseBuilder::id)
    /// - [`code`](PostV1PartnersGroupsCreateResponseBuilder::code)
    /// - [`name`](PostV1PartnersGroupsCreateResponseBuilder::name)
    /// - [`created_at`](PostV1PartnersGroupsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1PartnersGroupsCreateResponse, BuildError> {
        Ok(PostV1PartnersGroupsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
