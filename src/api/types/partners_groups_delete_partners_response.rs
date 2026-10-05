pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsDeletePartnersResponse {
    #[serde(default)]
    pub id: String,
}

impl GroupsDeletePartnersResponse {
    pub fn builder() -> GroupsDeletePartnersResponseBuilder {
        <GroupsDeletePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsDeletePartnersResponseBuilder {
    id: Option<String>,
}

impl GroupsDeletePartnersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsDeletePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GroupsDeletePartnersResponseBuilder::id)
    pub fn build(self) -> Result<GroupsDeletePartnersResponse, BuildError> {
        Ok(GroupsDeletePartnersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
