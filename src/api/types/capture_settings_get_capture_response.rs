pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetCaptureResponse {
    #[serde(rename = "intakeEnabled")]
    #[serde(default)]
    pub intake_enabled: bool,
    #[serde(rename = "captureAutoExtract")]
    #[serde(default)]
    pub capture_auto_extract: bool,
    #[serde(rename = "intakeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_address: Option<String>,
    #[serde(rename = "ocrConfigured")]
    #[serde(default)]
    pub ocr_configured: bool,
}

impl SettingsGetCaptureResponse {
    pub fn builder() -> SettingsGetCaptureResponseBuilder {
        <SettingsGetCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetCaptureResponseBuilder {
    intake_enabled: Option<bool>,
    capture_auto_extract: Option<bool>,
    intake_address: Option<String>,
    ocr_configured: Option<bool>,
}

impl SettingsGetCaptureResponseBuilder {
    pub fn intake_enabled(mut self, value: bool) -> Self {
        self.intake_enabled = Some(value);
        self
    }

    pub fn capture_auto_extract(mut self, value: bool) -> Self {
        self.capture_auto_extract = Some(value);
        self
    }

    pub fn intake_address(mut self, value: impl Into<String>) -> Self {
        self.intake_address = Some(value.into());
        self
    }

    pub fn ocr_configured(mut self, value: bool) -> Self {
        self.ocr_configured = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsGetCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`intake_enabled`](SettingsGetCaptureResponseBuilder::intake_enabled)
    /// - [`capture_auto_extract`](SettingsGetCaptureResponseBuilder::capture_auto_extract)
    /// - [`ocr_configured`](SettingsGetCaptureResponseBuilder::ocr_configured)
    pub fn build(self) -> Result<SettingsGetCaptureResponse, BuildError> {
        Ok(SettingsGetCaptureResponse {
            intake_enabled: self
                .intake_enabled
                .ok_or_else(|| BuildError::missing_field("intake_enabled"))?,
            capture_auto_extract: self
                .capture_auto_extract
                .ok_or_else(|| BuildError::missing_field("capture_auto_extract"))?,
            intake_address: self.intake_address,
            ocr_configured: self
                .ocr_configured
                .ok_or_else(|| BuildError::missing_field("ocr_configured"))?,
        })
    }
}
