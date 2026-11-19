pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountsCreatePartnersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// Sort code, routing number or other national bank code, for an account without an IBAN
    #[serde(rename = "bankCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    /// Account number, for an account without an IBAN
    #[serde(rename = "bankAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    /// Checking or savings account; a US ACH file needs it
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<BankAccountsCreatePartnersRequestAccountType>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// Defaults to the company's base currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "partnerId")]
    #[serde(default)]
    pub partner_id: String,
}

impl BankAccountsCreatePartnersRequest {
    pub fn builder() -> BankAccountsCreatePartnersRequestBuilder {
        <BankAccountsCreatePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountsCreatePartnersRequestBuilder {
    iban: Option<String>,
    bank_code: Option<String>,
    bank_account_number: Option<String>,
    account_type: Option<BankAccountsCreatePartnersRequestAccountType>,
    bank_name: Option<String>,
    bic: Option<String>,
    currency: Option<String>,
    is_default: Option<bool>,
    partner_id: Option<String>,
}

impl BankAccountsCreatePartnersRequestBuilder {
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

    pub fn account_type(mut self, value: BankAccountsCreatePartnersRequestAccountType) -> Self {
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

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BankAccountsCreatePartnersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`partner_id`](BankAccountsCreatePartnersRequestBuilder::partner_id)
    pub fn build(self) -> Result<BankAccountsCreatePartnersRequest, BuildError> {
        Ok(BankAccountsCreatePartnersRequest {
            iban: self.iban,
            bank_code: self.bank_code,
            bank_account_number: self.bank_account_number,
            account_type: self.account_type,
            bank_name: self.bank_name,
            bic: self.bic,
            currency: self.currency,
            is_default: self.is_default,
            partner_id: self
                .partner_id
                .ok_or_else(|| BuildError::missing_field("partner_id"))?,
        })
    }
}
