pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsSubmissionsMarkRequest {
    #[serde(default)]
    pub id: String,
    pub status: PostV1DeclarationsSubmissionsMarkRequestStatus,
    #[serde(rename = "externalRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl PostV1DeclarationsSubmissionsMarkRequest {
    pub fn builder() -> PostV1DeclarationsSubmissionsMarkRequestBuilder {
        <PostV1DeclarationsSubmissionsMarkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsSubmissionsMarkRequestBuilder {
    id: Option<String>,
    status: Option<PostV1DeclarationsSubmissionsMarkRequestStatus>,
    external_ref: Option<String>,
    message: Option<String>,
}

impl PostV1DeclarationsSubmissionsMarkRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1DeclarationsSubmissionsMarkRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn external_ref(mut self, value: impl Into<String>) -> Self {
        self.external_ref = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsSubmissionsMarkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsSubmissionsMarkRequestBuilder::id)
    /// - [`status`](PostV1DeclarationsSubmissionsMarkRequestBuilder::status)
    pub fn build(self) -> Result<PostV1DeclarationsSubmissionsMarkRequest, BuildError> {
        Ok(PostV1DeclarationsSubmissionsMarkRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            external_ref: self.external_ref,
            message: self.message,
        })
    }
}
