pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesHouseGetPartnersResponse {
    #[serde(rename = "companyNumber")]
    #[serde(default)]
    pub company_number: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    #[serde(rename = "incorporatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporated_on: Option<String>,
    #[serde(rename = "ceasedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ceased_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CompaniesHouseGetPartnersResponseAddress>,
    #[serde(rename = "accountingReferenceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_reference_date: Option<CompaniesHouseGetPartnersResponseAccountingReferenceDate>,
    #[serde(rename = "nextAccountsDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_accounts_due: Option<String>,
    #[serde(rename = "nextConfirmationStatementDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_confirmation_statement_due: Option<String>,
    #[serde(rename = "sicCodes")]
    #[serde(default)]
    pub sic_codes: Vec<String>,
}

impl CompaniesHouseGetPartnersResponse {
    pub fn builder() -> CompaniesHouseGetPartnersResponseBuilder {
        <CompaniesHouseGetPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesHouseGetPartnersResponseBuilder {
    company_number: Option<String>,
    name: Option<String>,
    status: Option<String>,
    r#type: Option<String>,
    jurisdiction: Option<String>,
    incorporated_on: Option<String>,
    ceased_on: Option<String>,
    address: Option<CompaniesHouseGetPartnersResponseAddress>,
    accounting_reference_date: Option<CompaniesHouseGetPartnersResponseAccountingReferenceDate>,
    next_accounts_due: Option<String>,
    next_confirmation_statement_due: Option<String>,
    sic_codes: Option<Vec<String>>,
}

impl CompaniesHouseGetPartnersResponseBuilder {
    pub fn company_number(mut self, value: impl Into<String>) -> Self {
        self.company_number = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn jurisdiction(mut self, value: impl Into<String>) -> Self {
        self.jurisdiction = Some(value.into());
        self
    }

    pub fn incorporated_on(mut self, value: impl Into<String>) -> Self {
        self.incorporated_on = Some(value.into());
        self
    }

    pub fn ceased_on(mut self, value: impl Into<String>) -> Self {
        self.ceased_on = Some(value.into());
        self
    }

    pub fn address(mut self, value: CompaniesHouseGetPartnersResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn accounting_reference_date(
        mut self,
        value: CompaniesHouseGetPartnersResponseAccountingReferenceDate,
    ) -> Self {
        self.accounting_reference_date = Some(value);
        self
    }

    pub fn next_accounts_due(mut self, value: impl Into<String>) -> Self {
        self.next_accounts_due = Some(value.into());
        self
    }

    pub fn next_confirmation_statement_due(mut self, value: impl Into<String>) -> Self {
        self.next_confirmation_statement_due = Some(value.into());
        self
    }

    pub fn sic_codes(mut self, value: Vec<String>) -> Self {
        self.sic_codes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesHouseGetPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_number`](CompaniesHouseGetPartnersResponseBuilder::company_number)
    /// - [`name`](CompaniesHouseGetPartnersResponseBuilder::name)
    /// - [`sic_codes`](CompaniesHouseGetPartnersResponseBuilder::sic_codes)
    pub fn build(self) -> Result<CompaniesHouseGetPartnersResponse, BuildError> {
        Ok(CompaniesHouseGetPartnersResponse {
            company_number: self
                .company_number
                .ok_or_else(|| BuildError::missing_field("company_number"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status,
            r#type: self.r#type,
            jurisdiction: self.jurisdiction,
            incorporated_on: self.incorporated_on,
            ceased_on: self.ceased_on,
            address: self.address,
            accounting_reference_date: self.accounting_reference_date,
            next_accounts_due: self.next_accounts_due,
            next_confirmation_statement_due: self.next_confirmation_statement_due,
            sic_codes: self
                .sic_codes
                .ok_or_else(|| BuildError::missing_field("sic_codes"))?,
        })
    }
}
