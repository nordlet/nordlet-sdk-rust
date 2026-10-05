pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsUpdateBankRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl AccountsUpdateBankRequest {
    pub fn builder() -> AccountsUpdateBankRequestBuilder {
        <AccountsUpdateBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsUpdateBankRequestBuilder {
    id: Option<String>,
    name: Option<String>,
    iban: Option<String>,
    account_code: Option<String>,
    is_active: Option<bool>,
}

impl AccountsUpdateBankRequestBuilder {
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

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountsUpdateBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountsUpdateBankRequestBuilder::id)
    pub fn build(self) -> Result<AccountsUpdateBankRequest, BuildError> {
        Ok(AccountsUpdateBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            iban: self.iban,
            account_code: self.account_code,
            is_active: self.is_active,
        })
    }
}
