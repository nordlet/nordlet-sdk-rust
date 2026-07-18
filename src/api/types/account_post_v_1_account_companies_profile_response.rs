pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesProfileResponse {
    #[serde(default)]
    pub id: String,
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
    #[serde(default)]
    pub is_vat_payer: bool,
    #[serde(rename = "isSandbox")]
    #[serde(default)]
    pub is_sandbox: bool,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(rename = "defaultInvoiceCurrency")]
    #[serde(default)]
    pub default_invoice_currency: String,
    pub status: PostV1AccountCompaniesProfileResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1AccountCompaniesProfileResponseAddress>,
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
    #[serde(rename = "logoFileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<String>,
}

impl PostV1AccountCompaniesProfileResponse {
    pub fn builder() -> PostV1AccountCompaniesProfileResponseBuilder {
        <PostV1AccountCompaniesProfileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesProfileResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    sme_exemption_number: Option<String>,
    is_vat_payer: Option<bool>,
    is_sandbox: Option<bool>,
    country_code: Option<String>,
    base_currency: Option<String>,
    default_invoice_currency: Option<String>,
    status: Option<PostV1AccountCompaniesProfileResponseStatus>,
    address: Option<PostV1AccountCompaniesProfileResponseAddress>,
    email: Option<String>,
    phone: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    peppol_id: Option<String>,
    logo_file_id: Option<String>,
}

impl PostV1AccountCompaniesProfileResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_currency = Some(value.into());
        self
    }

    pub fn default_invoice_currency(mut self, value: impl Into<String>) -> Self {
        self.default_invoice_currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1AccountCompaniesProfileResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn address(mut self, value: PostV1AccountCompaniesProfileResponseAddress) -> Self {
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

    pub fn logo_file_id(mut self, value: impl Into<String>) -> Self {
        self.logo_file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesProfileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountCompaniesProfileResponseBuilder::id)
    /// - [`name`](PostV1AccountCompaniesProfileResponseBuilder::name)
    /// - [`is_vat_payer`](PostV1AccountCompaniesProfileResponseBuilder::is_vat_payer)
    /// - [`is_sandbox`](PostV1AccountCompaniesProfileResponseBuilder::is_sandbox)
    /// - [`country_code`](PostV1AccountCompaniesProfileResponseBuilder::country_code)
    /// - [`base_currency`](PostV1AccountCompaniesProfileResponseBuilder::base_currency)
    /// - [`default_invoice_currency`](PostV1AccountCompaniesProfileResponseBuilder::default_invoice_currency)
    /// - [`status`](PostV1AccountCompaniesProfileResponseBuilder::status)
    pub fn build(self) -> Result<PostV1AccountCompaniesProfileResponse, BuildError> {
        Ok(PostV1AccountCompaniesProfileResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            sme_exemption_number: self.sme_exemption_number,
            is_vat_payer: self
                .is_vat_payer
                .ok_or_else(|| BuildError::missing_field("is_vat_payer"))?,
            is_sandbox: self
                .is_sandbox
                .ok_or_else(|| BuildError::missing_field("is_sandbox"))?,
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            default_invoice_currency: self
                .default_invoice_currency
                .ok_or_else(|| BuildError::missing_field("default_invoice_currency"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            address: self.address,
            email: self.email,
            phone: self.phone,
            iban: self.iban,
            bank_name: self.bank_name,
            peppol_id: self.peppol_id,
            logo_file_id: self.logo_file_id,
        })
    }
}
