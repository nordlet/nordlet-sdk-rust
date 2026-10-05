pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsListConsolidationRequest {}

impl GroupsListConsolidationRequest {
    pub fn builder() -> GroupsListConsolidationRequestBuilder {
        <GroupsListConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListConsolidationRequestBuilder {}

impl GroupsListConsolidationRequestBuilder {
    /// Consumes the builder and constructs a [`GroupsListConsolidationRequest`].
    pub fn build(self) -> Result<GroupsListConsolidationRequest, BuildError> {
        Ok(GroupsListConsolidationRequest {})
    }
}
