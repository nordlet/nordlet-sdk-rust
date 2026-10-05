pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeAccountResponseUser {
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
    #[serde(rename = "isSuperAdmin")]
    #[serde(default)]
    pub is_super_admin: bool,
}

impl MeAccountResponseUser {
    pub fn builder() -> MeAccountResponseUserBuilder {
        <MeAccountResponseUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeAccountResponseUserBuilder {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    locale: Option<String>,
    plan: Option<String>,
    is_super_admin: Option<bool>,
}

impl MeAccountResponseUserBuilder {
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

    pub fn is_super_admin(mut self, value: bool) -> Self {
        self.is_super_admin = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeAccountResponseUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MeAccountResponseUserBuilder::id)
    /// - [`email`](MeAccountResponseUserBuilder::email)
    /// - [`locale`](MeAccountResponseUserBuilder::locale)
    /// - [`plan`](MeAccountResponseUserBuilder::plan)
    /// - [`is_super_admin`](MeAccountResponseUserBuilder::is_super_admin)
    pub fn build(self) -> Result<MeAccountResponseUser, BuildError> {
        Ok(MeAccountResponseUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            locale: self
                .locale
                .ok_or_else(|| BuildError::missing_field("locale"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            is_super_admin: self
                .is_super_admin
                .ok_or_else(|| BuildError::missing_field("is_super_admin"))?,
        })
    }
}
