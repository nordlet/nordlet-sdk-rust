pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetCaptureRequest {}

impl SettingsGetCaptureRequest {
    pub fn builder() -> SettingsGetCaptureRequestBuilder {
        <SettingsGetCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetCaptureRequestBuilder {}

impl SettingsGetCaptureRequestBuilder {
    /// Consumes the builder and constructs a [`SettingsGetCaptureRequest`].
    pub fn build(self) -> Result<SettingsGetCaptureRequest, BuildError> {
        Ok(SettingsGetCaptureRequest {})
    }
}
