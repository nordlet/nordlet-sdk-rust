pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsListResponseRowsItem {
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
}

impl PostV1ConsolidationGroupsListResponseRowsItem {
    pub fn builder() -> PostV1ConsolidationGroupsListResponseRowsItemBuilder {
        <PostV1ConsolidationGroupsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsListResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    presentation_currency: Option<String>,
    member_count: Option<i64>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1ConsolidationGroupsListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ConsolidationGroupsListResponseRowsItemBuilder::id)
    /// - [`name`](PostV1ConsolidationGroupsListResponseRowsItemBuilder::name)
    /// - [`presentation_currency`](PostV1ConsolidationGroupsListResponseRowsItemBuilder::presentation_currency)
    /// - [`member_count`](PostV1ConsolidationGroupsListResponseRowsItemBuilder::member_count)
    /// - [`created_at`](PostV1ConsolidationGroupsListResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](PostV1ConsolidationGroupsListResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsListResponseRowsItem, BuildError> {
        Ok(PostV1ConsolidationGroupsListResponseRowsItem {
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
        })
    }
}
