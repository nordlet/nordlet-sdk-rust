pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureSettingsRegenerateIntakeRequest {}

impl PostV1CaptureSettingsRegenerateIntakeRequest {
    pub fn builder() -> PostV1CaptureSettingsRegenerateIntakeRequestBuilder {
        <PostV1CaptureSettingsRegenerateIntakeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureSettingsRegenerateIntakeRequestBuilder {}

impl PostV1CaptureSettingsRegenerateIntakeRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1CaptureSettingsRegenerateIntakeRequest`].
    pub fn build(self) -> Result<PostV1CaptureSettingsRegenerateIntakeRequest, BuildError> {
        Ok(PostV1CaptureSettingsRegenerateIntakeRequest {})
    }
}
