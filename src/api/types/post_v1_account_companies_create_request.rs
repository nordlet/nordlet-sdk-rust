pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesCreateRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "smeExemptionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sme_exemption_number: Option<String>,
    #[serde(rename = "isVatPayer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vat_payer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1AccountCompaniesCreateRequestAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "peppolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peppol_id: Option<String>,
    #[serde(rename = "defaultInvoiceCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_invoice_currency: Option<String>,
    /// Jurisdiction the company is registered in (immutable after creation)
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<PostV1AccountCompaniesCreateRequestCountryCode>,
    /// Sandbox companies hold test data and are purged immediately on delete (immutable after creation)
    #[serde(rename = "isSandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sandbox: Option<bool>,
}

impl PostV1AccountCompaniesCreateRequest {
    pub fn builder() -> PostV1AccountCompaniesCreateRequestBuilder {
        <PostV1AccountCompaniesCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesCreateRequestBuilder {
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    sme_exemption_number: Option<String>,
    is_vat_payer: Option<bool>,
    address: Option<PostV1AccountCompaniesCreateRequestAddress>,
    email: Option<String>,
    phone: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    peppol_id: Option<String>,
    default_invoice_currency: Option<String>,
    country_code: Option<PostV1AccountCompaniesCreateRequestCountryCode>,
    is_sandbox: Option<bool>,
}

impl PostV1AccountCompaniesCreateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn sme_exemption_number(mut self, value: impl Into<String>) -> Self {
        self.sme_exemption_number = Some(value.into());
        self
    }

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn address(mut self, value: PostV1AccountCompaniesCreateRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
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

    pub fn peppol_id(mut self, value: impl Into<String>) -> Self {
        self.peppol_id = Some(value.into());
        self
    }

    pub fn default_invoice_currency(mut self, value: impl Into<String>) -> Self {
        self.default_invoice_currency = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: PostV1AccountCompaniesCreateRequestCountryCode) -> Self {
        self.country_code = Some(value);
        self
    }

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1AccountCompaniesCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1AccountCompaniesCreateRequest, BuildError> {
        Ok(PostV1AccountCompaniesCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            sme_exemption_number: self.sme_exemption_number,
            is_vat_payer: self.is_vat_payer,
            address: self.address,
            email: self.email,
            phone: self.phone,
            iban: self.iban,
            bank_name: self.bank_name,
            peppol_id: self.peppol_id,
            default_invoice_currency: self.default_invoice_currency,
            country_code: self.country_code,
            is_sandbox: self.is_sandbox,
        })
    }
}
