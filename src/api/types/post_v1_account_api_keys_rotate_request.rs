pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysRotateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "overlapHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap_hours: Option<i64>,
    #[serde(rename = "expiresInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<i64>,
}

impl PostV1AccountApiKeysRotateRequest {
    pub fn builder() -> PostV1AccountApiKeysRotateRequestBuilder {
        <PostV1AccountApiKeysRotateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysRotateRequestBuilder {
    id: Option<String>,
    overlap_hours: Option<i64>,
    expires_in_days: Option<i64>,
}

impl PostV1AccountApiKeysRotateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn overlap_hours(mut self, value: i64) -> Self {
        self.overlap_hours = Some(value);
        self
    }

    pub fn expires_in_days(mut self, value: i64) -> Self {
        self.expires_in_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysRotateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountApiKeysRotateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1AccountApiKeysRotateRequest, BuildError> {
        Ok(PostV1AccountApiKeysRotateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            overlap_hours: self.overlap_hours,
            expires_in_days: self.expires_in_days,
        })
    }
}
