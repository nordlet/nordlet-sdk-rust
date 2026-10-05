pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalcPayrollResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "taxAllowance")]
    #[serde(default)]
    pub tax_allowance: String,
    #[serde(rename = "incomeTax")]
    #[serde(default)]
    pub income_tax: String,
    #[serde(rename = "employeeContributions")]
    #[serde(default)]
    pub employee_contributions: String,
    #[serde(rename = "employerContributions")]
    #[serde(default)]
    pub employer_contributions: String,
    #[serde(default)]
    pub components: Vec<CalcPayrollResponseComponentsItem>,
    #[serde(default)]
    pub net: String,
}

impl CalcPayrollResponse {
    pub fn builder() -> CalcPayrollResponseBuilder {
        <CalcPayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalcPayrollResponseBuilder {
    country_code: Option<String>,
    tax_allowance: Option<String>,
    income_tax: Option<String>,
    employee_contributions: Option<String>,
    employer_contributions: Option<String>,
    components: Option<Vec<CalcPayrollResponseComponentsItem>>,
    net: Option<String>,
}

impl CalcPayrollResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn tax_allowance(mut self, value: impl Into<String>) -> Self {
        self.tax_allowance = Some(value.into());
        self
    }

    pub fn income_tax(mut self, value: impl Into<String>) -> Self {
        self.income_tax = Some(value.into());
        self
    }

    pub fn employee_contributions(mut self, value: impl Into<String>) -> Self {
        self.employee_contributions = Some(value.into());
        self
    }

    pub fn employer_contributions(mut self, value: impl Into<String>) -> Self {
        self.employer_contributions = Some(value.into());
        self
    }

    pub fn components(mut self, value: Vec<CalcPayrollResponseComponentsItem>) -> Self {
        self.components = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalcPayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](CalcPayrollResponseBuilder::country_code)
    /// - [`tax_allowance`](CalcPayrollResponseBuilder::tax_allowance)
    /// - [`income_tax`](CalcPayrollResponseBuilder::income_tax)
    /// - [`employee_contributions`](CalcPayrollResponseBuilder::employee_contributions)
    /// - [`employer_contributions`](CalcPayrollResponseBuilder::employer_contributions)
    /// - [`components`](CalcPayrollResponseBuilder::components)
    /// - [`net`](CalcPayrollResponseBuilder::net)
    pub fn build(self) -> Result<CalcPayrollResponse, BuildError> {
        Ok(CalcPayrollResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            tax_allowance: self
                .tax_allowance
                .ok_or_else(|| BuildError::missing_field("tax_allowance"))?,
            income_tax: self
                .income_tax
                .ok_or_else(|| BuildError::missing_field("income_tax"))?,
            employee_contributions: self
                .employee_contributions
                .ok_or_else(|| BuildError::missing_field("employee_contributions"))?,
            employer_contributions: self
                .employer_contributions
                .ok_or_else(|| BuildError::missing_field("employer_contributions"))?,
            components: self
                .components
                .ok_or_else(|| BuildError::missing_field("components"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
