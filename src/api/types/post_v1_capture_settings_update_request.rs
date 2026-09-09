pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CaptureSettingsUpdateRequest {
    #[serde(rename = "intakeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_enabled: Option<bool>,
    #[serde(rename = "captureAutoExtract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_auto_extract: Option<bool>,
}

impl PostV1CaptureSettingsUpdateRequest {
    pub fn builder() -> PostV1CaptureSettingsUpdateRequestBuilder {
        <PostV1CaptureSettingsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CaptureSettingsUpdateRequestBuilder {
    intake_enabled: Option<bool>,
    capture_auto_extract: Option<bool>,
}

impl PostV1CaptureSettingsUpdateRequestBuilder {
    pub fn intake_enabled(mut self, value: bool) -> Self {
        self.intake_enabled = Some(value);
        self
    }

    pub fn capture_auto_extract(mut self, value: bool) -> Self {
        self.capture_auto_extract = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CaptureSettingsUpdateRequest`].
    pub fn build(self) -> Result<PostV1CaptureSettingsUpdateRequest, BuildError> {
        Ok(PostV1CaptureSettingsUpdateRequest {
            intake_enabled: self.intake_enabled,
            capture_auto_extract: self.capture_auto_extract,
        })
    }
}
