pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesViewSalesResponseSeller {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default)]
    pub address: InvoicesViewSalesResponseSellerAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "bankBic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_bic: Option<String>,
    #[serde(rename = "bankCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(rename = "bankBranchCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_code: Option<String>,
    #[serde(rename = "bankAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
}

impl InvoicesViewSalesResponseSeller {
    pub fn builder() -> InvoicesViewSalesResponseSellerBuilder {
        <InvoicesViewSalesResponseSellerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesViewSalesResponseSellerBuilder {
    name: Option<String>,
    vat_code: Option<String>,
    country_code: Option<String>,
    address: Option<InvoicesViewSalesResponseSellerAddress>,
    email: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    bank_bic: Option<String>,
    bank_code: Option<String>,
    bank_branch_code: Option<String>,
    bank_account_number: Option<String>,
}

impl InvoicesViewSalesResponseSellerBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn address(mut self, value: InvoicesViewSalesResponseSellerAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn bank_bic(mut self, value: impl Into<String>) -> Self {
        self.bank_bic = Some(value.into());
        self
    }

    pub fn bank_code(mut self, value: impl Into<String>) -> Self {
        self.bank_code = Some(value.into());
        self
    }

    pub fn bank_branch_code(mut self, value: impl Into<String>) -> Self {
        self.bank_branch_code = Some(value.into());
        self
    }

    pub fn bank_account_number(mut self, value: impl Into<String>) -> Self {
        self.bank_account_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesViewSalesResponseSeller`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](InvoicesViewSalesResponseSellerBuilder::name)
    /// - [`address`](InvoicesViewSalesResponseSellerBuilder::address)
    pub fn build(self) -> Result<InvoicesViewSalesResponseSeller, BuildError> {
        Ok(InvoicesViewSalesResponseSeller {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            vat_code: self.vat_code,
            country_code: self.country_code,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            email: self.email,
            iban: self.iban,
            bank_name: self.bank_name,
            bank_bic: self.bank_bic,
            bank_code: self.bank_code,
            bank_branch_code: self.bank_branch_code,
            bank_account_number: self.bank_account_number,
        })
    }
}
