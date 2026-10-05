pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsGetConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl GroupsGetConsolidationRequest {
    pub fn builder() -> GroupsGetConsolidationRequestBuilder {
        <GroupsGetConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsGetConsolidationRequestBuilder {
    group_id: Option<String>,
}

impl GroupsGetConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsGetConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](GroupsGetConsolidationRequestBuilder::group_id)
    pub fn build(self) -> Result<GroupsGetConsolidationRequest, BuildError> {
        Ok(GroupsGetConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
