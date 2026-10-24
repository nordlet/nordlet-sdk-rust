pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersUpcomingPartnersRequest {}

impl DebtRemindersUpcomingPartnersRequest {
    pub fn builder() -> DebtRemindersUpcomingPartnersRequestBuilder {
        <DebtRemindersUpcomingPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersUpcomingPartnersRequestBuilder {}

impl DebtRemindersUpcomingPartnersRequestBuilder {
    /// Consumes the builder and constructs a [`DebtRemindersUpcomingPartnersRequest`].
    pub fn build(self) -> Result<DebtRemindersUpcomingPartnersRequest, BuildError> {
        Ok(DebtRemindersUpcomingPartnersRequest {})
    }
}
