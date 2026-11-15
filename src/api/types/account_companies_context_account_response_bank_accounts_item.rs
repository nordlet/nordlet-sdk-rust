pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountResponseBankAccountsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
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
}

impl CompaniesContextAccountResponseBankAccountsItem {
    pub fn builder() -> CompaniesContextAccountResponseBankAccountsItemBuilder {
        <CompaniesContextAccountResponseBankAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponseBankAccountsItemBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    iban: Option<String>,
    currency: Option<String>,
    account_code: Option<String>,
    is_active: Option<bool>,
}

impl CompaniesContextAccountResponseBankAccountsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
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

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponseBankAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompaniesContextAccountResponseBankAccountsItemBuilder::id)
    /// - [`name`](CompaniesContextAccountResponseBankAccountsItemBuilder::name)
    /// - [`r#type`](CompaniesContextAccountResponseBankAccountsItemBuilder::r#type)
    /// - [`currency`](CompaniesContextAccountResponseBankAccountsItemBuilder::currency)
    /// - [`account_code`](CompaniesContextAccountResponseBankAccountsItemBuilder::account_code)
    /// - [`is_active`](CompaniesContextAccountResponseBankAccountsItemBuilder::is_active)
    pub fn build(self) -> Result<CompaniesContextAccountResponseBankAccountsItem, BuildError> {
        Ok(CompaniesContextAccountResponseBankAccountsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
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
        })
    }
}
