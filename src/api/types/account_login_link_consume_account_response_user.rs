pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoginLinkConsumeAccountResponseUser {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub plan: String,
}

impl LoginLinkConsumeAccountResponseUser {
    pub fn builder() -> LoginLinkConsumeAccountResponseUserBuilder {
        <LoginLinkConsumeAccountResponseUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoginLinkConsumeAccountResponseUserBuilder {
    id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    plan: Option<String>,
}

impl LoginLinkConsumeAccountResponseUserBuilder {
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

    pub fn plan(mut self, value: impl Into<String>) -> Self {
        self.plan = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LoginLinkConsumeAccountResponseUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LoginLinkConsumeAccountResponseUserBuilder::id)
    /// - [`email`](LoginLinkConsumeAccountResponseUserBuilder::email)
    /// - [`plan`](LoginLinkConsumeAccountResponseUserBuilder::plan)
    pub fn build(self) -> Result<LoginLinkConsumeAccountResponseUser, BuildError> {
        Ok(LoginLinkConsumeAccountResponseUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
        })
    }
}
