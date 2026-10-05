pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsCreatePartnersResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl GroupsCreatePartnersResponse {
    pub fn builder() -> GroupsCreatePartnersResponseBuilder {
        <GroupsCreatePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsCreatePartnersResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl GroupsCreatePartnersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupsCreatePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GroupsCreatePartnersResponseBuilder::id)
    /// - [`code`](GroupsCreatePartnersResponseBuilder::code)
    /// - [`name`](GroupsCreatePartnersResponseBuilder::name)
    /// - [`created_at`](GroupsCreatePartnersResponseBuilder::created_at)
    pub fn build(self) -> Result<GroupsCreatePartnersResponse, BuildError> {
        Ok(GroupsCreatePartnersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
