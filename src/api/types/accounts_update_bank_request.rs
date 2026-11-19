pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsUpdateBankRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AccountsUpdateBankRequestType>,
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
    /// What the bank gave the company for payment files from this account: the Bacs service user number for a UK account, the ACH Immediate Origin and Company Identification for a US account
    #[serde(rename = "paymentFileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_file_settings: Option<AccountsUpdateBankRequestPaymentFileSettings>,
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
    r#type: Option<AccountsUpdateBankRequestType>,
    iban: Option<String>,
    bank_code: Option<String>,
    bank_account_number: Option<String>,
    payment_file_settings: Option<AccountsUpdateBankRequestPaymentFileSettings>,
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

    pub fn r#type(mut self, value: AccountsUpdateBankRequestType) -> Self {
        self.r#type = Some(value);
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

    pub fn payment_file_settings(
        mut self,
        value: AccountsUpdateBankRequestPaymentFileSettings,
    ) -> Self {
        self.payment_file_settings = Some(value);
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
            r#type: self.r#type,
            iban: self.iban,
            bank_code: self.bank_code,
            bank_account_number: self.bank_account_number,
            payment_file_settings: self.payment_file_settings,
            account_code: self.account_code,
            is_active: self.is_active,
        })
    }
}
