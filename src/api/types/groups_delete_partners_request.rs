pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsDeletePartnersRequest {
    #[serde(default)]
    pub id: String,
}

impl GroupsDeletePartnersRequest {
    pub fn builder() -> GroupsDeletePartnersRequestBuilder {
        <GroupsDeletePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsDeletePartnersRequestBuilder {
    id: Option<String>,
}

impl GroupsDeletePartnersRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsDeletePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GroupsDeletePartnersRequestBuilder::id)
    pub fn build(self) -> Result<GroupsDeletePartnersRequest, BuildError> {
        Ok(GroupsDeletePartnersRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
