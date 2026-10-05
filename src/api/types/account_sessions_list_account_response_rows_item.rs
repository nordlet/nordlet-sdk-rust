pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsListAccountResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "companyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_seen_at: Option<DateTime<FixedOffset>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub current: bool,
}

impl SessionsListAccountResponseRowsItem {
    pub fn builder() -> SessionsListAccountResponseRowsItemBuilder {
        <SessionsListAccountResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsListAccountResponseRowsItemBuilder {
    id: Option<String>,
    company_id: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    last_seen_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    expires_at: Option<DateTime<FixedOffset>>,
    current: Option<bool>,
}

impl SessionsListAccountResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn current(mut self, value: bool) -> Self {
        self.current = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsListAccountResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SessionsListAccountResponseRowsItemBuilder::id)
    /// - [`created_at`](SessionsListAccountResponseRowsItemBuilder::created_at)
    /// - [`expires_at`](SessionsListAccountResponseRowsItemBuilder::expires_at)
    /// - [`current`](SessionsListAccountResponseRowsItemBuilder::current)
    pub fn build(self) -> Result<SessionsListAccountResponseRowsItem, BuildError> {
        Ok(SessionsListAccountResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            company_id: self.company_id,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            last_seen_at: self.last_seen_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
        })
    }
}
