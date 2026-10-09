pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsUpdateAgreementsResponse {
    #[serde(rename = "autoBilling")]
    #[serde(default)]
    pub auto_billing: bool,
}

impl SettingsUpdateAgreementsResponse {
    pub fn builder() -> SettingsUpdateAgreementsResponseBuilder {
        <SettingsUpdateAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateAgreementsResponseBuilder {
    auto_billing: Option<bool>,
}

impl SettingsUpdateAgreementsResponseBuilder {
    pub fn auto_billing(mut self, value: bool) -> Self {
        self.auto_billing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_billing`](SettingsUpdateAgreementsResponseBuilder::auto_billing)
    pub fn build(self) -> Result<SettingsUpdateAgreementsResponse, BuildError> {
        Ok(SettingsUpdateAgreementsResponse {
            auto_billing: self
                .auto_billing
                .ok_or_else(|| BuildError::missing_field("auto_billing"))?,
        })
    }
}
