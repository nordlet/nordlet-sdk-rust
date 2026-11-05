pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseAuthEventsItem {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ExportAccountResponseAuthEventsItem {
    pub fn builder() -> ExportAccountResponseAuthEventsItemBuilder {
        <ExportAccountResponseAuthEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseAuthEventsItemBuilder {
    action: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseAuthEventsItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseAuthEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ExportAccountResponseAuthEventsItemBuilder::action)
    /// - [`created_at`](ExportAccountResponseAuthEventsItemBuilder::created_at)
    pub fn build(self) -> Result<ExportAccountResponseAuthEventsItem, BuildError> {
        Ok(ExportAccountResponseAuthEventsItem {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
