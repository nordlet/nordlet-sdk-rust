pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyLinksRemoveRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(default)]
    pub id: String,
}

impl PostV1ConsolidationIntercompanyLinksRemoveRequest {
    pub fn builder() -> PostV1ConsolidationIntercompanyLinksRemoveRequestBuilder {
        <PostV1ConsolidationIntercompanyLinksRemoveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyLinksRemoveRequestBuilder {
    group_id: Option<String>,
    id: Option<String>,
}

impl PostV1ConsolidationIntercompanyLinksRemoveRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyLinksRemoveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationIntercompanyLinksRemoveRequestBuilder::group_id)
    /// - [`id`](PostV1ConsolidationIntercompanyLinksRemoveRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyLinksRemoveRequest, BuildError> {
        Ok(PostV1ConsolidationIntercompanyLinksRemoveRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
