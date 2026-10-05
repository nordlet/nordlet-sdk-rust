pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationListDeclarationsRequest {}

impl AutomationListDeclarationsRequest {
    pub fn builder() -> AutomationListDeclarationsRequestBuilder {
        <AutomationListDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationListDeclarationsRequestBuilder {}

impl AutomationListDeclarationsRequestBuilder {
    /// Consumes the builder and constructs a [`AutomationListDeclarationsRequest`].
    pub fn build(self) -> Result<AutomationListDeclarationsRequest, BuildError> {
        Ok(AutomationListDeclarationsRequest {})
    }
}
