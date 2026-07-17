pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountCompaniesUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(rename = "isVatPayer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_vat_payer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1AccountCompaniesUpdateRequestAddress>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<PostV1AccountCompaniesUpdateRequestLogo>,
}

impl PostV1AccountCompaniesUpdateRequest {
    pub fn builder() -> PostV1AccountCompaniesUpdateRequestBuilder {
        <PostV1AccountCompaniesUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesUpdateRequestBuilder {
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    is_vat_payer: Option<bool>,
    address: Option<PostV1AccountCompaniesUpdateRequestAddress>,
    email: Option<String>,
    phone: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    peppol_id: Option<String>,
    default_invoice_currency: Option<String>,
    logo: Option<PostV1AccountCompaniesUpdateRequestLogo>,
}

impl PostV1AccountCompaniesUpdateRequestBuilder {
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

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn address(mut self, value: PostV1AccountCompaniesUpdateRequestAddress) -> Self {
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

    pub fn logo(mut self, value: PostV1AccountCompaniesUpdateRequestLogo) -> Self {
        self.logo = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesUpdateRequest`].
    pub fn build(self) -> Result<PostV1AccountCompaniesUpdateRequest, BuildError> {
        Ok(PostV1AccountCompaniesUpdateRequest {
            name: self.name,
            code: self.code,
            vat_code: self.vat_code,
            is_vat_payer: self.is_vat_payer,
            address: self.address,
            email: self.email,
            phone: self.phone,
            iban: self.iban,
            bank_name: self.bank_name,
            peppol_id: self.peppol_id,
            default_invoice_currency: self.default_invoice_currency,
            logo: self.logo,
        })
    }
}
