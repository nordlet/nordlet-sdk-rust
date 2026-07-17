pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1ConsolidationMembersAddRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "memberCompanyId")]
    #[serde(default)]
    pub member_company_id: String,
    #[serde(rename = "ownershipPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PostV1ConsolidationMembersAddRequestMethod>,
}

impl PostV1ConsolidationMembersAddRequest {
    pub fn builder() -> PostV1ConsolidationMembersAddRequestBuilder {
        <PostV1ConsolidationMembersAddRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationMembersAddRequestBuilder {
    group_id: Option<String>,
    member_company_id: Option<String>,
    ownership_percent: Option<f64>,
    method: Option<PostV1ConsolidationMembersAddRequestMethod>,
}

impl PostV1ConsolidationMembersAddRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn member_company_id(mut self, value: impl Into<String>) -> Self {
        self.member_company_id = Some(value.into());
        self
    }

    pub fn ownership_percent(mut self, value: f64) -> Self {
        self.ownership_percent = Some(value);
        self
    }

    pub fn method(mut self, value: PostV1ConsolidationMembersAddRequestMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationMembersAddRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationMembersAddRequestBuilder::group_id)
    /// - [`member_company_id`](PostV1ConsolidationMembersAddRequestBuilder::member_company_id)
    pub fn build(self) -> Result<PostV1ConsolidationMembersAddRequest, BuildError> {
        Ok(PostV1ConsolidationMembersAddRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            member_company_id: self
                .member_company_id
                .ok_or_else(|| BuildError::missing_field("member_company_id"))?,
            ownership_percent: self.ownership_percent,
            method: self.method,
        })
    }
}
