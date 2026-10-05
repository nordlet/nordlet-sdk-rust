pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersPreviewPartnersRequest {}

impl DebtRemindersPreviewPartnersRequest {
    pub fn builder() -> DebtRemindersPreviewPartnersRequestBuilder {
        <DebtRemindersPreviewPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersPreviewPartnersRequestBuilder {}

impl DebtRemindersPreviewPartnersRequestBuilder {
    /// Consumes the builder and constructs a [`DebtRemindersPreviewPartnersRequest`].
    pub fn build(self) -> Result<DebtRemindersPreviewPartnersRequest, BuildError> {
        Ok(DebtRemindersPreviewPartnersRequest {})
    }
}
