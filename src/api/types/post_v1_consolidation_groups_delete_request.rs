pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsDeleteRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl PostV1ConsolidationGroupsDeleteRequest {
    pub fn builder() -> PostV1ConsolidationGroupsDeleteRequestBuilder {
        <PostV1ConsolidationGroupsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsDeleteRequestBuilder {
    group_id: Option<String>,
}

impl PostV1ConsolidationGroupsDeleteRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationGroupsDeleteRequestBuilder::group_id)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsDeleteRequest, BuildError> {
        Ok(PostV1ConsolidationGroupsDeleteRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
