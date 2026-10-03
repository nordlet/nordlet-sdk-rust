pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1OfficersUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub role: PostV1OfficersUpdateResponseRole,
    #[serde(rename = "personalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    #[serde(rename = "appointedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointed_on: Option<String>,
    #[serde(rename = "powerNotary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_notary: Option<String>,
    #[serde(rename = "resignedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resigned_on: Option<String>,
    #[serde(rename = "signsAccounts")]
    #[serde(default)]
    pub signs_accounts: bool,
}

impl PostV1OfficersUpdateResponse {
    pub fn builder() -> PostV1OfficersUpdateResponseBuilder {
        <PostV1OfficersUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OfficersUpdateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    role: Option<PostV1OfficersUpdateResponseRole>,
    personal_code: Option<String>,
    birth_date: Option<String>,
    appointed_on: Option<String>,
    power_notary: Option<String>,
    resigned_on: Option<String>,
    signs_accounts: Option<bool>,
}

impl PostV1OfficersUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: PostV1OfficersUpdateResponseRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn personal_code(mut self, value: impl Into<String>) -> Self {
        self.personal_code = Some(value.into());
        self
    }

    pub fn birth_date(mut self, value: impl Into<String>) -> Self {
        self.birth_date = Some(value.into());
        self
    }

    pub fn appointed_on(mut self, value: impl Into<String>) -> Self {
        self.appointed_on = Some(value.into());
        self
    }

    pub fn power_notary(mut self, value: impl Into<String>) -> Self {
        self.power_notary = Some(value.into());
        self
    }

    pub fn resigned_on(mut self, value: impl Into<String>) -> Self {
        self.resigned_on = Some(value.into());
        self
    }

    pub fn signs_accounts(mut self, value: bool) -> Self {
        self.signs_accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1OfficersUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OfficersUpdateResponseBuilder::id)
    /// - [`name`](PostV1OfficersUpdateResponseBuilder::name)
    /// - [`role`](PostV1OfficersUpdateResponseBuilder::role)
    /// - [`signs_accounts`](PostV1OfficersUpdateResponseBuilder::signs_accounts)
    pub fn build(self) -> Result<PostV1OfficersUpdateResponse, BuildError> {
        Ok(PostV1OfficersUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            personal_code: self.personal_code,
            birth_date: self.birth_date,
            appointed_on: self.appointed_on,
            power_notary: self.power_notary,
            resigned_on: self.resigned_on,
            signs_accounts: self
                .signs_accounts
                .ok_or_else(|| BuildError::missing_field("signs_accounts"))?,
        })
    }
}
