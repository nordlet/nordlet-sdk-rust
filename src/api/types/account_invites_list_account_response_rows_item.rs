pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitesListAccountResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub expired: bool,
}

impl InvitesListAccountResponseRowsItem {
    pub fn builder() -> InvitesListAccountResponseRowsItemBuilder {
        <InvitesListAccountResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitesListAccountResponseRowsItemBuilder {
    id: Option<String>,
    email: Option<String>,
    role: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    expired: Option<bool>,
}

impl InvitesListAccountResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitesListAccountResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitesListAccountResponseRowsItemBuilder::id)
    /// - [`email`](InvitesListAccountResponseRowsItemBuilder::email)
    /// - [`role`](InvitesListAccountResponseRowsItemBuilder::role)
    /// - [`expires_at`](InvitesListAccountResponseRowsItemBuilder::expires_at)
    /// - [`created_at`](InvitesListAccountResponseRowsItemBuilder::created_at)
    /// - [`expired`](InvitesListAccountResponseRowsItemBuilder::expired)
    pub fn build(self) -> Result<InvitesListAccountResponseRowsItem, BuildError> {
        Ok(InvitesListAccountResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
        })
    }
}
