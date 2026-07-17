pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl PostV1PartnersStatusesUpdateRequest {
    pub fn builder() -> PostV1PartnersStatusesUpdateRequestBuilder {
        <PostV1PartnersStatusesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    sort_order: Option<i64>,
}

impl PostV1PartnersStatusesUpdateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PartnersStatusesUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1PartnersStatusesUpdateRequest, BuildError> {
        Ok(PostV1PartnersStatusesUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            sort_order: self.sort_order,
        })
    }
}
