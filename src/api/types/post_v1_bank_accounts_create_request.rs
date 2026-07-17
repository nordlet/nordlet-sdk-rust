pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankAccountsCreateRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
}

impl PostV1BankAccountsCreateRequest {
    pub fn builder() -> PostV1BankAccountsCreateRequestBuilder {
        <PostV1BankAccountsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankAccountsCreateRequestBuilder {
    name: Option<String>,
    iban: Option<String>,
    currency: Option<String>,
    account_code: Option<String>,
}

impl PostV1BankAccountsCreateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1BankAccountsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1BankAccountsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1BankAccountsCreateRequest, BuildError> {
        Ok(PostV1BankAccountsCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            iban: self.iban,
            currency: self.currency,
            account_code: self.account_code,
        })
    }
}
