pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetAssetsRequest {}

impl SettingsGetAssetsRequest {
    pub fn builder() -> SettingsGetAssetsRequestBuilder {
        <SettingsGetAssetsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetAssetsRequestBuilder {}

impl SettingsGetAssetsRequestBuilder {
    /// Consumes the builder and constructs a [`SettingsGetAssetsRequest`].
    pub fn build(self) -> Result<SettingsGetAssetsRequest, BuildError> {
        Ok(SettingsGetAssetsRequest {})
    }
}
