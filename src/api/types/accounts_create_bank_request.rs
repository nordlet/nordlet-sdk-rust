pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsCreateBankRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AccountsCreateBankRequestType>,
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
    pub payment_file_settings: Option<AccountsCreateBankRequestPaymentFileSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "accountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "documentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ref: Option<String>,
}

impl AccountsCreateBankRequest {
    pub fn builder() -> AccountsCreateBankRequestBuilder {
        <AccountsCreateBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsCreateBankRequestBuilder {
    name: Option<String>,
    r#type: Option<AccountsCreateBankRequestType>,
    iban: Option<String>,
    bank_code: Option<String>,
    bank_account_number: Option<String>,
    payment_file_settings: Option<AccountsCreateBankRequestPaymentFileSettings>,
    currency: Option<String>,
    account_code: Option<String>,
    document_ref: Option<String>,
}

impl AccountsCreateBankRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AccountsCreateBankRequestType) -> Self {
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
        value: AccountsCreateBankRequestPaymentFileSettings,
    ) -> Self {
        self.payment_file_settings = Some(value);
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

    pub fn document_ref(mut self, value: impl Into<String>) -> Self {
        self.document_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsCreateBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AccountsCreateBankRequestBuilder::name)
    pub fn build(self) -> Result<AccountsCreateBankRequest, BuildError> {
        Ok(AccountsCreateBankRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
            iban: self.iban,
            bank_code: self.bank_code,
            bank_account_number: self.bank_account_number,
            payment_file_settings: self.payment_file_settings,
            currency: self.currency,
            account_code: self.account_code,
            document_ref: self.document_ref,
        })
    }
}
