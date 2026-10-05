pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersListAccountRequest {}

impl MembersListAccountRequest {
    pub fn builder() -> MembersListAccountRequestBuilder {
        <MembersListAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersListAccountRequestBuilder {}

impl MembersListAccountRequestBuilder {
    /// Consumes the builder and constructs a [`MembersListAccountRequest`].
    pub fn build(self) -> Result<MembersListAccountRequest, BuildError> {
        Ok(MembersListAccountRequest {})
    }
}
