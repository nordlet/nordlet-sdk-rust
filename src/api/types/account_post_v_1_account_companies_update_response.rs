pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AccountCompaniesUpdateResponse {
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
    /// Chart of accounts template the company was seeded with
    #[serde(rename = "chartTemplate")]
    #[serde(default)]
    pub chart_template: String,
    /// Chart of accounts template of the company country
    #[serde(rename = "countryChartTemplate")]
    #[serde(default)]
    pub country_chart_template: String,
    #[serde(rename = "baseCurrency")]
    #[serde(default)]
    pub base_currency: String,
    #[serde(rename = "defaultInvoiceCurrency")]
    #[serde(default)]
    pub default_invoice_currency: String,
    pub status: PostV1AccountCompaniesUpdateResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostV1AccountCompaniesUpdateResponseAddress>,
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
    #[serde(rename = "logoFileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_file_id: Option<String>,
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
    pub accounts_kept_by: Option<PostV1AccountCompaniesUpdateResponseAccountsKeptBy>,
    #[serde(rename = "vatPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_period: Option<PostV1AccountCompaniesUpdateResponseVatPeriod>,
    #[serde(rename = "fiscalYearEndMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end_month: Option<i64>,
    #[serde(rename = "timeZone")]
    #[serde(default)]
    pub time_zone: String,
    #[serde(rename = "filingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filing_options: Option<HashMap<String, Option<String>>>,
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
    #[serde(default)]
    pub audit_required: bool,
}

impl PostV1AccountCompaniesUpdateResponse {
    pub fn builder() -> PostV1AccountCompaniesUpdateResponseBuilder {
        <PostV1AccountCompaniesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountCompaniesUpdateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    sme_exemption_number: Option<String>,
    is_vat_payer: Option<bool>,
    is_sandbox: Option<bool>,
    country_code: Option<String>,
    chart_template: Option<String>,
    country_chart_template: Option<String>,
    base_currency: Option<String>,
    default_invoice_currency: Option<String>,
    status: Option<PostV1AccountCompaniesUpdateResponseStatus>,
    address: Option<PostV1AccountCompaniesUpdateResponseAddress>,
    email: Option<String>,
    phone: Option<String>,
    iban: Option<String>,
    bank_name: Option<String>,
    peppol_id: Option<String>,
    sepa_creditor_id: Option<String>,
    logo_file_id: Option<String>,
    legal_form: Option<String>,
    registry_name: Option<String>,
    incorporated_on: Option<String>,
    share_capital: Option<String>,
    accounts_kept_by: Option<PostV1AccountCompaniesUpdateResponseAccountsKeptBy>,
    vat_period: Option<PostV1AccountCompaniesUpdateResponseVatPeriod>,
    fiscal_year_end_month: Option<i64>,
    time_zone: Option<String>,
    filing_options: Option<HashMap<String, Option<String>>>,
    bookkeeper_name: Option<String>,
    auditor_name: Option<String>,
    auditor_registration_number: Option<String>,
    audit_required: Option<bool>,
}

impl PostV1AccountCompaniesUpdateResponseBuilder {
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

    pub fn chart_template(mut self, value: impl Into<String>) -> Self {
        self.chart_template = Some(value.into());
        self
    }

    pub fn country_chart_template(mut self, value: impl Into<String>) -> Self {
        self.country_chart_template = Some(value.into());
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

    pub fn status(mut self, value: PostV1AccountCompaniesUpdateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn address(mut self, value: PostV1AccountCompaniesUpdateResponseAddress) -> Self {
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

    pub fn logo_file_id(mut self, value: impl Into<String>) -> Self {
        self.logo_file_id = Some(value.into());
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
        value: PostV1AccountCompaniesUpdateResponseAccountsKeptBy,
    ) -> Self {
        self.accounts_kept_by = Some(value);
        self
    }

    pub fn vat_period(mut self, value: PostV1AccountCompaniesUpdateResponseVatPeriod) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1AccountCompaniesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AccountCompaniesUpdateResponseBuilder::id)
    /// - [`name`](PostV1AccountCompaniesUpdateResponseBuilder::name)
    /// - [`is_vat_payer`](PostV1AccountCompaniesUpdateResponseBuilder::is_vat_payer)
    /// - [`is_sandbox`](PostV1AccountCompaniesUpdateResponseBuilder::is_sandbox)
    /// - [`country_code`](PostV1AccountCompaniesUpdateResponseBuilder::country_code)
    /// - [`chart_template`](PostV1AccountCompaniesUpdateResponseBuilder::chart_template)
    /// - [`country_chart_template`](PostV1AccountCompaniesUpdateResponseBuilder::country_chart_template)
    /// - [`base_currency`](PostV1AccountCompaniesUpdateResponseBuilder::base_currency)
    /// - [`default_invoice_currency`](PostV1AccountCompaniesUpdateResponseBuilder::default_invoice_currency)
    /// - [`status`](PostV1AccountCompaniesUpdateResponseBuilder::status)
    /// - [`time_zone`](PostV1AccountCompaniesUpdateResponseBuilder::time_zone)
    /// - [`audit_required`](PostV1AccountCompaniesUpdateResponseBuilder::audit_required)
    pub fn build(self) -> Result<PostV1AccountCompaniesUpdateResponse, BuildError> {
        Ok(PostV1AccountCompaniesUpdateResponse {
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
            chart_template: self
                .chart_template
                .ok_or_else(|| BuildError::missing_field("chart_template"))?,
            country_chart_template: self
                .country_chart_template
                .ok_or_else(|| BuildError::missing_field("country_chart_template"))?,
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
            sepa_creditor_id: self.sepa_creditor_id,
            logo_file_id: self.logo_file_id,
            legal_form: self.legal_form,
            registry_name: self.registry_name,
            incorporated_on: self.incorporated_on,
            share_capital: self.share_capital,
            accounts_kept_by: self.accounts_kept_by,
            vat_period: self.vat_period,
            fiscal_year_end_month: self.fiscal_year_end_month,
            time_zone: self
                .time_zone
                .ok_or_else(|| BuildError::missing_field("time_zone"))?,
            filing_options: self.filing_options,
            bookkeeper_name: self.bookkeeper_name,
            auditor_name: self.auditor_name,
            auditor_registration_number: self.auditor_registration_number,
            audit_required: self
                .audit_required
                .ok_or_else(|| BuildError::missing_field("audit_required"))?,
        })
    }
}
