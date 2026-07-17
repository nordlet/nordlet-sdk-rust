pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationMembersRemoveRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "memberCompanyId")]
    #[serde(default)]
    pub member_company_id: String,
}

impl PostV1ConsolidationMembersRemoveRequest {
    pub fn builder() -> PostV1ConsolidationMembersRemoveRequestBuilder {
        <PostV1ConsolidationMembersRemoveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationMembersRemoveRequestBuilder {
    group_id: Option<String>,
    member_company_id: Option<String>,
}

impl PostV1ConsolidationMembersRemoveRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn member_company_id(mut self, value: impl Into<String>) -> Self {
        self.member_company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationMembersRemoveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationMembersRemoveRequestBuilder::group_id)
    /// - [`member_company_id`](PostV1ConsolidationMembersRemoveRequestBuilder::member_company_id)
    pub fn build(self) -> Result<PostV1ConsolidationMembersRemoveRequest, BuildError> {
        Ok(PostV1ConsolidationMembersRemoveRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            member_company_id: self
                .member_company_id
                .ok_or_else(|| BuildError::missing_field("member_company_id"))?,
        })
    }
}
