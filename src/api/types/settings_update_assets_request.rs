pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsUpdateAssetsRequest {
    #[serde(rename = "autoDepreciation")]
    #[serde(default)]
    pub auto_depreciation: bool,
}

impl SettingsUpdateAssetsRequest {
    pub fn builder() -> SettingsUpdateAssetsRequestBuilder {
        <SettingsUpdateAssetsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateAssetsRequestBuilder {
    auto_depreciation: Option<bool>,
}

impl SettingsUpdateAssetsRequestBuilder {
    pub fn auto_depreciation(mut self, value: bool) -> Self {
        self.auto_depreciation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateAssetsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_depreciation`](SettingsUpdateAssetsRequestBuilder::auto_depreciation)
    pub fn build(self) -> Result<SettingsUpdateAssetsRequest, BuildError> {
        Ok(SettingsUpdateAssetsRequest {
            auto_depreciation: self
                .auto_depreciation
                .ok_or_else(|| BuildError::missing_field("auto_depreciation"))?,
        })
    }
}
