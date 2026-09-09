pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureSettingsGetRequest {}

impl PostV1CaptureSettingsGetRequest {
    pub fn builder() -> PostV1CaptureSettingsGetRequestBuilder {
        <PostV1CaptureSettingsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureSettingsGetRequestBuilder {}

impl PostV1CaptureSettingsGetRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CaptureSettingsGetRequest`].
    pub fn build(self) -> Result<PostV1CaptureSettingsGetRequest, BuildError> {
        Ok(PostV1CaptureSettingsGetRequest {})
    }
}
