pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsUpdateAgreementsRequest {
    #[serde(rename = "autoBilling")]
    #[serde(default)]
    pub auto_billing: bool,
}

impl SettingsUpdateAgreementsRequest {
    pub fn builder() -> SettingsUpdateAgreementsRequestBuilder {
        <SettingsUpdateAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateAgreementsRequestBuilder {
    auto_billing: Option<bool>,
}

impl SettingsUpdateAgreementsRequestBuilder {
    pub fn auto_billing(mut self, value: bool) -> Self {
        self.auto_billing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_billing`](SettingsUpdateAgreementsRequestBuilder::auto_billing)
    pub fn build(self) -> Result<SettingsUpdateAgreementsRequest, BuildError> {
        Ok(SettingsUpdateAgreementsRequest {
            auto_billing: self
                .auto_billing
                .ok_or_else(|| BuildError::missing_field("auto_billing"))?,
        })
    }
}
