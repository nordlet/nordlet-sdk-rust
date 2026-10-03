pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsSubmissionsRetryRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1DeclarationsSubmissionsRetryRequest {
    pub fn builder() -> PostV1DeclarationsSubmissionsRetryRequestBuilder {
        <PostV1DeclarationsSubmissionsRetryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsSubmissionsRetryRequestBuilder {
    id: Option<String>,
}

impl PostV1DeclarationsSubmissionsRetryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsSubmissionsRetryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1DeclarationsSubmissionsRetryRequestBuilder::id)
    pub fn build(self) -> Result<PostV1DeclarationsSubmissionsRetryRequest, BuildError> {
        Ok(PostV1DeclarationsSubmissionsRetryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
