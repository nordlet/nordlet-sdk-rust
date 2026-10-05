pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesListAccountRequest {}

impl InvitesListAccountRequest {
    pub fn builder() -> InvitesListAccountRequestBuilder {
        <InvitesListAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesListAccountRequestBuilder {}

impl InvitesListAccountRequestBuilder {
    /// Consumes the builder and constructs a [`InvitesListAccountRequest`].
    pub fn build(self) -> Result<InvitesListAccountRequest, BuildError> {
        Ok(InvitesListAccountRequest {})
    }
}
