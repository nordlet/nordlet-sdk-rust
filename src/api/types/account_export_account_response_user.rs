pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportAccountResponseUser {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub locale: String,
    #[serde(default)]
    pub plan: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ExportAccountResponseUser {
    pub fn builder() -> ExportAccountResponseUserBuilder {
        <ExportAccountResponseUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportAccountResponseUserBuilder {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    locale: Option<String>,
    plan: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ExportAccountResponseUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn plan(mut self, value: impl Into<String>) -> Self {
        self.plan = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportAccountResponseUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExportAccountResponseUserBuilder::id)
    /// - [`email`](ExportAccountResponseUserBuilder::email)
    /// - [`locale`](ExportAccountResponseUserBuilder::locale)
    /// - [`plan`](ExportAccountResponseUserBuilder::plan)
    /// - [`created_at`](ExportAccountResponseUserBuilder::created_at)
    pub fn build(self) -> Result<ExportAccountResponseUser, BuildError> {
        Ok(ExportAccountResponseUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
