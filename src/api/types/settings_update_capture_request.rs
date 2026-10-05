pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsUpdateCaptureRequest {
    #[serde(rename = "intakeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_enabled: Option<bool>,
    #[serde(rename = "captureAutoExtract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_auto_extract: Option<bool>,
}

impl SettingsUpdateCaptureRequest {
    pub fn builder() -> SettingsUpdateCaptureRequestBuilder {
        <SettingsUpdateCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateCaptureRequestBuilder {
    intake_enabled: Option<bool>,
    capture_auto_extract: Option<bool>,
}

impl SettingsUpdateCaptureRequestBuilder {
    pub fn intake_enabled(mut self, value: bool) -> Self {
        self.intake_enabled = Some(value);
        self
    }

    pub fn capture_auto_extract(mut self, value: bool) -> Self {
        self.capture_auto_extract = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateCaptureRequest`].
    pub fn build(self) -> Result<SettingsUpdateCaptureRequest, BuildError> {
        Ok(SettingsUpdateCaptureRequest {
            intake_enabled: self.intake_enabled,
            capture_auto_extract: self.capture_auto_extract,
        })
    }
}
