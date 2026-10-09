pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsGetAgreementsRequest {}

impl SettingsGetAgreementsRequest {
    pub fn builder() -> SettingsGetAgreementsRequestBuilder {
        <SettingsGetAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetAgreementsRequestBuilder {}

impl SettingsGetAgreementsRequestBuilder {
    /// Consumes the builder and constructs a [`SettingsGetAgreementsRequest`].
    pub fn build(self) -> Result<SettingsGetAgreementsRequest, BuildError> {
        Ok(SettingsGetAgreementsRequest {})
    }
}
