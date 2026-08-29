pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyCandidatesRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl PostV1ConsolidationIntercompanyCandidatesRequest {
    pub fn builder() -> PostV1ConsolidationIntercompanyCandidatesRequestBuilder {
        <PostV1ConsolidationIntercompanyCandidatesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyCandidatesRequestBuilder {
    group_id: Option<String>,
}

impl PostV1ConsolidationIntercompanyCandidatesRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyCandidatesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationIntercompanyCandidatesRequestBuilder::group_id)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyCandidatesRequest, BuildError> {
        Ok(PostV1ConsolidationIntercompanyCandidatesRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
