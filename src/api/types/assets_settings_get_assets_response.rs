pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetAssetsResponse {
    #[serde(rename = "autoDepreciation")]
    #[serde(default)]
    pub auto_depreciation: bool,
}

impl SettingsGetAssetsResponse {
    pub fn builder() -> SettingsGetAssetsResponseBuilder {
        <SettingsGetAssetsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetAssetsResponseBuilder {
    auto_depreciation: Option<bool>,
}

impl SettingsGetAssetsResponseBuilder {
    pub fn auto_depreciation(mut self, value: bool) -> Self {
        self.auto_depreciation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsGetAssetsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_depreciation`](SettingsGetAssetsResponseBuilder::auto_depreciation)
    pub fn build(self) -> Result<SettingsGetAssetsResponse, BuildError> {
        Ok(SettingsGetAssetsResponse {
            auto_depreciation: self
                .auto_depreciation
                .ok_or_else(|| BuildError::missing_field("auto_depreciation"))?,
        })
    }
}
