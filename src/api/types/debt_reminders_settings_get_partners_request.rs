pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersSettingsGetPartnersRequest {}

impl DebtRemindersSettingsGetPartnersRequest {
    pub fn builder() -> DebtRemindersSettingsGetPartnersRequestBuilder {
        <DebtRemindersSettingsGetPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersSettingsGetPartnersRequestBuilder {}

impl DebtRemindersSettingsGetPartnersRequestBuilder {
    /// Consumes the builder and constructs a [`DebtRemindersSettingsGetPartnersRequest`].
    pub fn build(self) -> Result<DebtRemindersSettingsGetPartnersRequest, BuildError> {
        Ok(DebtRemindersSettingsGetPartnersRequest {})
    }
}
