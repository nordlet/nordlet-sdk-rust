pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountExportResponseUser {
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
    pub created_at: String,
}

impl PostV1AccountExportResponseUser {
    pub fn builder() -> PostV1AccountExportResponseUserBuilder {
        <PostV1AccountExportResponseUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountExportResponseUserBuilder {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    locale: Option<String>,
    plan: Option<String>,
    created_at: Option<String>,
}

impl PostV1AccountExportResponseUserBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountExportResponseUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountExportResponseUserBuilder::id)
    /// - [`email`](PostV1AccountExportResponseUserBuilder::email)
    /// - [`locale`](PostV1AccountExportResponseUserBuilder::locale)
    /// - [`plan`](PostV1AccountExportResponseUserBuilder::plan)
    /// - [`created_at`](PostV1AccountExportResponseUserBuilder::created_at)
    pub fn build(self) -> Result<PostV1AccountExportResponseUser, BuildError> {
        Ok(PostV1AccountExportResponseUser {
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
