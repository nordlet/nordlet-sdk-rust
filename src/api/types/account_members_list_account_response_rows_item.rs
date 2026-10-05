pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersListAccountResponseRowsItem {
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl MembersListAccountResponseRowsItem {
    pub fn builder() -> MembersListAccountResponseRowsItemBuilder {
        <MembersListAccountResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersListAccountResponseRowsItemBuilder {
    user_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    role: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl MembersListAccountResponseRowsItemBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersListAccountResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](MembersListAccountResponseRowsItemBuilder::user_id)
    /// - [`email`](MembersListAccountResponseRowsItemBuilder::email)
    /// - [`role`](MembersListAccountResponseRowsItemBuilder::role)
    /// - [`created_at`](MembersListAccountResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<MembersListAccountResponseRowsItem, BuildError> {
        Ok(MembersListAccountResponseRowsItem {
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
