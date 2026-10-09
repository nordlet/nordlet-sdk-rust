pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsUpdateAssetsResponse {
    #[serde(rename = "autoDepreciation")]
    #[serde(default)]
    pub auto_depreciation: bool,
}

impl SettingsUpdateAssetsResponse {
    pub fn builder() -> SettingsUpdateAssetsResponseBuilder {
        <SettingsUpdateAssetsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateAssetsResponseBuilder {
    auto_depreciation: Option<bool>,
}

impl SettingsUpdateAssetsResponseBuilder {
    pub fn auto_depreciation(mut self, value: bool) -> Self {
        self.auto_depreciation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateAssetsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_depreciation`](SettingsUpdateAssetsResponseBuilder::auto_depreciation)
    pub fn build(self) -> Result<SettingsUpdateAssetsResponse, BuildError> {
        Ok(SettingsUpdateAssetsResponse {
            auto_depreciation: self
                .auto_depreciation
                .ok_or_else(|| BuildError::missing_field("auto_depreciation"))?,
        })
    }
}
