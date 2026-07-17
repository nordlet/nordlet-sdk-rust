pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1PartnersStatusesCreateResponse {
    pub fn builder() -> PostV1PartnersStatusesCreateResponseBuilder {
        <PostV1PartnersStatusesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    sort_order: Option<i64>,
    created_at: Option<String>,
}

impl PostV1PartnersStatusesCreateResponseBuilder {
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

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersStatusesCreateResponseBuilder::id)
    /// - [`code`](PostV1PartnersStatusesCreateResponseBuilder::code)
    /// - [`name`](PostV1PartnersStatusesCreateResponseBuilder::name)
    /// - [`sort_order`](PostV1PartnersStatusesCreateResponseBuilder::sort_order)
    /// - [`created_at`](PostV1PartnersStatusesCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1PartnersStatusesCreateResponse, BuildError> {
        Ok(PostV1PartnersStatusesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
