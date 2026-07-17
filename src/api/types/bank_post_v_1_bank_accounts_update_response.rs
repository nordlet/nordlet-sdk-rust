pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankAccountsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1BankAccountsUpdateResponse {
    pub fn builder() -> PostV1BankAccountsUpdateResponseBuilder {
        <PostV1BankAccountsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankAccountsUpdateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    iban: Option<String>,
    currency: Option<String>,
    account_code: Option<String>,
    is_active: Option<bool>,
    created_at: Option<String>,
}

impl PostV1BankAccountsUpdateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankAccountsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankAccountsUpdateResponseBuilder::id)
    /// - [`name`](PostV1BankAccountsUpdateResponseBuilder::name)
    /// - [`currency`](PostV1BankAccountsUpdateResponseBuilder::currency)
    /// - [`account_code`](PostV1BankAccountsUpdateResponseBuilder::account_code)
    /// - [`is_active`](PostV1BankAccountsUpdateResponseBuilder::is_active)
    /// - [`created_at`](PostV1BankAccountsUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1BankAccountsUpdateResponse, BuildError> {
        Ok(PostV1BankAccountsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            iban: self.iban,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
