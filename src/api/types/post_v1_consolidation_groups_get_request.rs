pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsGetRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl PostV1ConsolidationGroupsGetRequest {
    pub fn builder() -> PostV1ConsolidationGroupsGetRequestBuilder {
        <PostV1ConsolidationGroupsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsGetRequestBuilder {
    group_id: Option<String>,
}

impl PostV1ConsolidationGroupsGetRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationGroupsGetRequestBuilder::group_id)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsGetRequest, BuildError> {
        Ok(PostV1ConsolidationGroupsGetRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
