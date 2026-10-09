pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetAgreementsResponse {
    #[serde(rename = "autoBilling")]
    #[serde(default)]
    pub auto_billing: bool,
}

impl SettingsGetAgreementsResponse {
    pub fn builder() -> SettingsGetAgreementsResponseBuilder {
        <SettingsGetAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetAgreementsResponseBuilder {
    auto_billing: Option<bool>,
}

impl SettingsGetAgreementsResponseBuilder {
    pub fn auto_billing(mut self, value: bool) -> Self {
        self.auto_billing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsGetAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_billing`](SettingsGetAgreementsResponseBuilder::auto_billing)
    pub fn build(self) -> Result<SettingsGetAgreementsResponse, BuildError> {
        Ok(SettingsGetAgreementsResponse {
            auto_billing: self
                .auto_billing
                .ok_or_else(|| BuildError::missing_field("auto_billing"))?,
        })
    }
}
