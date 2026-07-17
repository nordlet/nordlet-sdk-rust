pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl PostV1PartnersStatusesCreateRequest {
    pub fn builder() -> PostV1PartnersStatusesCreateRequestBuilder {
        <PostV1PartnersStatusesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    sort_order: Option<i64>,
}

impl PostV1PartnersStatusesCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PartnersStatusesCreateRequestBuilder::code)
    /// - [`name`](PostV1PartnersStatusesCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1PartnersStatusesCreateRequest, BuildError> {
        Ok(PostV1PartnersStatusesCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            sort_order: self.sort_order,
        })
    }
}
