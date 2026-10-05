pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsDeleteConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl GroupsDeleteConsolidationRequest {
    pub fn builder() -> GroupsDeleteConsolidationRequestBuilder {
        <GroupsDeleteConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsDeleteConsolidationRequestBuilder {
    group_id: Option<String>,
}

impl GroupsDeleteConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsDeleteConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](GroupsDeleteConsolidationRequestBuilder::group_id)
    pub fn build(self) -> Result<GroupsDeleteConsolidationRequest, BuildError> {
        Ok(GroupsDeleteConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
