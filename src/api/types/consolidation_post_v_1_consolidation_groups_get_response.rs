pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsGetResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "presentationCurrency")]
    #[serde(default)]
    pub presentation_currency: String,
    #[serde(rename = "memberCount")]
    #[serde(default)]
    pub member_count: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub members: Vec<PostV1ConsolidationGroupsGetResponseMembersItem>,
}

impl PostV1ConsolidationGroupsGetResponse {
    pub fn builder() -> PostV1ConsolidationGroupsGetResponseBuilder {
        <PostV1ConsolidationGroupsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsGetResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    presentation_currency: Option<String>,
    member_count: Option<i64>,
    created_at: Option<String>,
    updated_at: Option<String>,
    members: Option<Vec<PostV1ConsolidationGroupsGetResponseMembersItem>>,
}

impl PostV1ConsolidationGroupsGetResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn presentation_currency(mut self, value: impl Into<String>) -> Self {
        self.presentation_currency = Some(value.into());
        self
    }

    pub fn member_count(mut self, value: i64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<PostV1ConsolidationGroupsGetResponseMembersItem>) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ConsolidationGroupsGetResponseBuilder::id)
    /// - [`name`](PostV1ConsolidationGroupsGetResponseBuilder::name)
    /// - [`presentation_currency`](PostV1ConsolidationGroupsGetResponseBuilder::presentation_currency)
    /// - [`member_count`](PostV1ConsolidationGroupsGetResponseBuilder::member_count)
    /// - [`created_at`](PostV1ConsolidationGroupsGetResponseBuilder::created_at)
    /// - [`updated_at`](PostV1ConsolidationGroupsGetResponseBuilder::updated_at)
    /// - [`members`](PostV1ConsolidationGroupsGetResponseBuilder::members)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsGetResponse, BuildError> {
        Ok(PostV1ConsolidationGroupsGetResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            presentation_currency: self
                .presentation_currency
                .ok_or_else(|| BuildError::missing_field("presentation_currency"))?,
            member_count: self
                .member_count
                .ok_or_else(|| BuildError::missing_field("member_count"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
        })
    }
}
