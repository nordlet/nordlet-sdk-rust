pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1AccountCompaniesUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[serde(rename = "vatPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_period: Option<PostV1AccountCompaniesUpdateRequestVatPeriod>,
    #[serde(rename = "fiscalYearEndMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end_month: Option<i64>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "filingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filing_options: Option<HashMap<String, Option<String>>>,
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
    #[serde(rename = "sepaCreditorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_creditor_id: Option<String>,
    #[serde(rename = "defaultInvoiceCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_invoice_currency: Option<String>,
    #[serde(rename = "legalForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_form: Option<String>,
    #[serde(rename = "registryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(rename = "incorporatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporated_on: Option<String>,
    #[serde(rename = "shareCapital")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_capital: Option<String>,
    #[serde(rename = "accountsKeptBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_kept_by: Option<PostV1AccountCompaniesUpdateRequestAccountsKeptBy>,
    #[serde(rename = "bookkeeperName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookkeeper_name: Option<String>,
    #[serde(rename = "auditorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_name: Option<String>,
    #[serde(rename = "auditorRegistrationNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_registration_number: Option<String>,
    #[serde(rename = "auditRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_required: Option<bool>,
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
    sme_exemption_number: Option<String>,
    is_vat_payer: Option<bool>,
    vat_period: Option<PostV1AccountCompaniesUpdateRequestVatPeriod>,
    fiscal_year_end_month: Option<i64>,
    time_zone: Option<String>,
    filing_options: Option<HashMap<String, Option<String>>>,
    address: Option<PostV1AccountCompaniesUpdateRequestAddress>,
    email: Option<String>,
    phone: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    peppol_id: Option<String>,
    sepa_creditor_id: Option<String>,
    default_invoice_currency: Option<String>,
    legal_form: Option<String>,
    registry_name: Option<String>,
    incorporated_on: Option<String>,
    share_capital: Option<String>,
    accounts_kept_by: Option<PostV1AccountCompaniesUpdateRequestAccountsKeptBy>,
    bookkeeper_name: Option<String>,
    auditor_name: Option<String>,
    auditor_registration_number: Option<String>,
    audit_required: Option<bool>,
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

    pub fn sme_exemption_number(mut self, value: impl Into<String>) -> Self {
        self.sme_exemption_number = Some(value.into());
        self
    }

    pub fn is_vat_payer(mut self, value: bool) -> Self {
        self.is_vat_payer = Some(value);
        self
    }

    pub fn vat_period(mut self, value: PostV1AccountCompaniesUpdateRequestVatPeriod) -> Self {
        self.vat_period = Some(value);
        self
    }

    pub fn fiscal_year_end_month(mut self, value: i64) -> Self {
        self.fiscal_year_end_month = Some(value);
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn filing_options(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.filing_options = Some(value);
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

    pub fn sepa_creditor_id(mut self, value: impl Into<String>) -> Self {
        self.sepa_creditor_id = Some(value.into());
        self
    }

    pub fn default_invoice_currency(mut self, value: impl Into<String>) -> Self {
        self.default_invoice_currency = Some(value.into());
        self
    }

    pub fn legal_form(mut self, value: impl Into<String>) -> Self {
        self.legal_form = Some(value.into());
        self
    }

    pub fn registry_name(mut self, value: impl Into<String>) -> Self {
        self.registry_name = Some(value.into());
        self
    }

    pub fn incorporated_on(mut self, value: impl Into<String>) -> Self {
        self.incorporated_on = Some(value.into());
        self
    }

    pub fn share_capital(mut self, value: impl Into<String>) -> Self {
        self.share_capital = Some(value.into());
        self
    }

    pub fn accounts_kept_by(
        mut self,
        value: PostV1AccountCompaniesUpdateRequestAccountsKeptBy,
    ) -> Self {
        self.accounts_kept_by = Some(value);
        self
    }

    pub fn bookkeeper_name(mut self, value: impl Into<String>) -> Self {
        self.bookkeeper_name = Some(value.into());
        self
    }

    pub fn auditor_name(mut self, value: impl Into<String>) -> Self {
        self.auditor_name = Some(value.into());
        self
    }

    pub fn auditor_registration_number(mut self, value: impl Into<String>) -> Self {
        self.auditor_registration_number = Some(value.into());
        self
    }

    pub fn audit_required(mut self, value: bool) -> Self {
        self.audit_required = Some(value);
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
            sme_exemption_number: self.sme_exemption_number,
            is_vat_payer: self.is_vat_payer,
            vat_period: self.vat_period,
            fiscal_year_end_month: self.fiscal_year_end_month,
            time_zone: self.time_zone,
            filing_options: self.filing_options,
            address: self.address,
            email: self.email,
            phone: self.phone,
            iban: self.iban,
            bank_name: self.bank_name,
            peppol_id: self.peppol_id,
            sepa_creditor_id: self.sepa_creditor_id,
            default_invoice_currency: self.default_invoice_currency,
            legal_form: self.legal_form,
            registry_name: self.registry_name,
            incorporated_on: self.incorporated_on,
            share_capital: self.share_capital,
            accounts_kept_by: self.accounts_kept_by,
            bookkeeper_name: self.bookkeeper_name,
            auditor_name: self.auditor_name,
            auditor_registration_number: self.auditor_registration_number,
            audit_required: self.audit_required,
            logo: self.logo,
        })
    }
}
