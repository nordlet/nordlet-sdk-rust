pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateOfficersRequest {
    #[serde(default)]
    pub name: String,
    pub role: CreateOfficersRequestRole,
    #[serde(rename = "personalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,
    #[serde(rename = "appointedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointed_on: Option<NaiveDate>,
    #[serde(rename = "powerNotary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_notary: Option<String>,
    #[serde(rename = "resignedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resigned_on: Option<NaiveDate>,
    #[serde(rename = "signsAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signs_accounts: Option<bool>,
}

impl CreateOfficersRequest {
    pub fn builder() -> CreateOfficersRequestBuilder {
        <CreateOfficersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOfficersRequestBuilder {
    name: Option<String>,
    role: Option<CreateOfficersRequestRole>,
    personal_code: Option<String>,
    birth_date: Option<NaiveDate>,
    appointed_on: Option<NaiveDate>,
    power_notary: Option<String>,
    resigned_on: Option<NaiveDate>,
    signs_accounts: Option<bool>,
}

impl CreateOfficersRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: CreateOfficersRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn personal_code(mut self, value: impl Into<String>) -> Self {
        self.personal_code = Some(value.into());
        self
    }

    pub fn birth_date(mut self, value: NaiveDate) -> Self {
        self.birth_date = Some(value);
        self
    }

    pub fn appointed_on(mut self, value: NaiveDate) -> Self {
        self.appointed_on = Some(value);
        self
    }

    pub fn power_notary(mut self, value: impl Into<String>) -> Self {
        self.power_notary = Some(value.into());
        self
    }

    pub fn resigned_on(mut self, value: NaiveDate) -> Self {
        self.resigned_on = Some(value);
        self
    }

    pub fn signs_accounts(mut self, value: bool) -> Self {
        self.signs_accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOfficersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateOfficersRequestBuilder::name)
    /// - [`role`](CreateOfficersRequestBuilder::role)
    pub fn build(self) -> Result<CreateOfficersRequest, BuildError> {
        Ok(CreateOfficersRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            personal_code: self.personal_code,
            birth_date: self.birth_date,
            appointed_on: self.appointed_on,
            power_notary: self.power_notary,
            resigned_on: self.resigned_on,
            signs_accounts: self.signs_accounts,
        })
    }
}
