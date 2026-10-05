pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsRegenerateIntakeCaptureRequest {}

impl SettingsRegenerateIntakeCaptureRequest {
    pub fn builder() -> SettingsRegenerateIntakeCaptureRequestBuilder {
        <SettingsRegenerateIntakeCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsRegenerateIntakeCaptureRequestBuilder {}

impl SettingsRegenerateIntakeCaptureRequestBuilder {
    /// Consumes the builder and constructs a [`SettingsRegenerateIntakeCaptureRequest`].
    pub fn build(self) -> Result<SettingsRegenerateIntakeCaptureRequest, BuildError> {
        Ok(SettingsRegenerateIntakeCaptureRequest {})
    }
}
