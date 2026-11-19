pub use crate::prelude::*;

/// What the bank gave the company for payment files from this account: the Bacs service user number for a UK account, the ACH Immediate Origin and Company Identification for a US account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountsUpdateBankRequestPaymentFileSettings {
    /// Bacs: the service user number (SUN) of the company for this account
    #[serde(rename = "serviceUserNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_user_number: Option<String>,
    /// Bacs: the service user's short name; defaults to the company name
    #[serde(rename = "serviceUserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_user_name: Option<String>,
    /// Nacha: the routing number of the bank that receives the file; defaults to the account's bankCode
    #[serde(rename = "odfiRoutingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odfi_routing_number: Option<String>,
    /// Nacha: the Immediate Origin the bank gave the company
    #[serde(rename = "immediateOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_origin: Option<String>,
    /// Nacha: the name of the bank that receives the file
    #[serde(rename = "immediateDestinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_destination_name: Option<String>,
    /// Nacha: the Company Identification the bank gave the company
    #[serde(rename = "companyIdentification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_identification: Option<String>,
    /// Nacha: the company name shown to the receiver; defaults to the company name
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Nacha: the Company Discretionary Data the bank asks for
    #[serde(rename = "companyDiscretionaryData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_discretionary_data: Option<String>,
}

impl AccountsUpdateBankRequestPaymentFileSettings {
    pub fn builder() -> AccountsUpdateBankRequestPaymentFileSettingsBuilder {
        <AccountsUpdateBankRequestPaymentFileSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountsUpdateBankRequestPaymentFileSettingsBuilder {
    service_user_number: Option<String>,
    service_user_name: Option<String>,
    odfi_routing_number: Option<String>,
    immediate_origin: Option<String>,
    immediate_destination_name: Option<String>,
    company_identification: Option<String>,
    company_name: Option<String>,
    company_discretionary_data: Option<String>,
}

impl AccountsUpdateBankRequestPaymentFileSettingsBuilder {
    pub fn service_user_number(mut self, value: impl Into<String>) -> Self {
        self.service_user_number = Some(value.into());
        self
    }

    pub fn service_user_name(mut self, value: impl Into<String>) -> Self {
        self.service_user_name = Some(value.into());
        self
    }

    pub fn odfi_routing_number(mut self, value: impl Into<String>) -> Self {
        self.odfi_routing_number = Some(value.into());
        self
    }

    pub fn immediate_origin(mut self, value: impl Into<String>) -> Self {
        self.immediate_origin = Some(value.into());
        self
    }

    pub fn immediate_destination_name(mut self, value: impl Into<String>) -> Self {
        self.immediate_destination_name = Some(value.into());
        self
    }

    pub fn company_identification(mut self, value: impl Into<String>) -> Self {
        self.company_identification = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn company_discretionary_data(mut self, value: impl Into<String>) -> Self {
        self.company_discretionary_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountsUpdateBankRequestPaymentFileSettings`].
    pub fn build(self) -> Result<AccountsUpdateBankRequestPaymentFileSettings, BuildError> {
        Ok(AccountsUpdateBankRequestPaymentFileSettings {
            service_user_number: self.service_user_number,
            service_user_name: self.service_user_name,
            odfi_routing_number: self.odfi_routing_number,
            immediate_origin: self.immediate_origin,
            immediate_destination_name: self.immediate_destination_name,
            company_identification: self.company_identification,
            company_name: self.company_name,
            company_discretionary_data: self.company_discretionary_data,
        })
    }
}
