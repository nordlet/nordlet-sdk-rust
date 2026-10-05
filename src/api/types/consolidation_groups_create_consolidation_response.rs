pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsCreateConsolidationResponse {
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
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl GroupsCreateConsolidationResponse {
    pub fn builder() -> GroupsCreateConsolidationResponseBuilder {
        <GroupsCreateConsolidationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsCreateConsolidationResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    presentation_currency: Option<String>,
    member_count: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl GroupsCreateConsolidationResponseBuilder {
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsCreateConsolidationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GroupsCreateConsolidationResponseBuilder::id)
    /// - [`name`](GroupsCreateConsolidationResponseBuilder::name)
    /// - [`presentation_currency`](GroupsCreateConsolidationResponseBuilder::presentation_currency)
    /// - [`member_count`](GroupsCreateConsolidationResponseBuilder::member_count)
    /// - [`created_at`](GroupsCreateConsolidationResponseBuilder::created_at)
    /// - [`updated_at`](GroupsCreateConsolidationResponseBuilder::updated_at)
    pub fn build(self) -> Result<GroupsCreateConsolidationResponse, BuildError> {
        Ok(GroupsCreateConsolidationResponse {
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
