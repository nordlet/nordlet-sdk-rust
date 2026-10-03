pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1OfficersCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub role: PostV1OfficersCreateResponseRole,
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

impl PostV1OfficersCreateResponse {
    pub fn builder() -> PostV1OfficersCreateResponseBuilder {
        <PostV1OfficersCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OfficersCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    role: Option<PostV1OfficersCreateResponseRole>,
    personal_code: Option<String>,
    birth_date: Option<String>,
    appointed_on: Option<String>,
    power_notary: Option<String>,
    resigned_on: Option<String>,
    signs_accounts: Option<bool>,
}

impl PostV1OfficersCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: PostV1OfficersCreateResponseRole) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1OfficersCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1OfficersCreateResponseBuilder::id)
    /// - [`name`](PostV1OfficersCreateResponseBuilder::name)
    /// - [`role`](PostV1OfficersCreateResponseBuilder::role)
    /// - [`signs_accounts`](PostV1OfficersCreateResponseBuilder::signs_accounts)
    pub fn build(self) -> Result<PostV1OfficersCreateResponse, BuildError> {
        Ok(PostV1OfficersCreateResponse {
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
