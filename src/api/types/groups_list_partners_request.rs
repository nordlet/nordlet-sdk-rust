pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsListPartnersRequest {}

impl GroupsListPartnersRequest {
    pub fn builder() -> GroupsListPartnersRequestBuilder {
        <GroupsListPartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsListPartnersRequestBuilder {}

impl GroupsListPartnersRequestBuilder {
    /// Consumes the builder and constructs a [`GroupsListPartnersRequest`].
    pub fn build(self) -> Result<GroupsListPartnersRequest, BuildError> {
        Ok(GroupsListPartnersRequest {})
    }
}
