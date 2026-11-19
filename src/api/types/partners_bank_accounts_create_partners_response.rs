pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountsCreatePartnersResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bankCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(rename = "bankAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    /// Checking or savings account; a US ACH file needs it
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<BankAccountsCreatePartnersResponseAccountType>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl BankAccountsCreatePartnersResponse {
    pub fn builder() -> BankAccountsCreatePartnersResponseBuilder {
        <BankAccountsCreatePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsCreatePartnersResponseBuilder {
    id: Option<String>,
    partner_id: Option<String>,
    iban: Option<String>,
    bank_code: Option<String>,
    bank_account_number: Option<String>,
    account_type: Option<BankAccountsCreatePartnersResponseAccountType>,
    bank_name: Option<String>,
    bic: Option<String>,
    currency: Option<String>,
    is_default: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl BankAccountsCreatePartnersResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn bank_account_number(mut self, value: impl Into<String>) -> Self {
        self.bank_account_number = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: BankAccountsCreatePartnersResponseAccountType) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsCreatePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BankAccountsCreatePartnersResponseBuilder::id)
    /// - [`partner_id`](BankAccountsCreatePartnersResponseBuilder::partner_id)
    /// - [`currency`](BankAccountsCreatePartnersResponseBuilder::currency)
    /// - [`is_default`](BankAccountsCreatePartnersResponseBuilder::is_default)
    /// - [`created_at`](BankAccountsCreatePartnersResponseBuilder::created_at)
    pub fn build(self) -> Result<BankAccountsCreatePartnersResponse, BuildError> {
        Ok(BankAccountsCreatePartnersResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
            iban: self.iban,
            bank_code: self.bank_code,
            bank_account_number: self.bank_account_number,
            account_type: self.account_type,
            bank_name: self.bank_name,
            bic: self.bic,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
