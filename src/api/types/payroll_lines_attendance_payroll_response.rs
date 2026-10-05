pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinesAttendancePayrollResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "contractId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    #[serde(rename = "employeeName")]
    #[serde(default)]
    pub employee_name: String,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub natura: String,
    #[serde(default)]
    pub additions: Vec<LinesAttendancePayrollResponseAdditionsItem>,
    #[serde(default)]
    pub deductions: Vec<LinesAttendancePayrollResponseDeductionsItem>,
    #[serde(rename = "taxableBase")]
    #[serde(default)]
    pub taxable_base: String,
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
    pub components: Vec<LinesAttendancePayrollResponseComponentsItem>,
    #[serde(default)]
    pub net: String,
    #[serde(rename = "daysWorked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_worked: Option<String>,
    #[serde(rename = "hoursWorked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_worked: Option<String>,
    #[serde(rename = "registeredDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_days: Option<String>,
    #[serde(rename = "averageHourlyEarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_hourly_earnings: Option<String>,
}

impl LinesAttendancePayrollResponse {
    pub fn builder() -> LinesAttendancePayrollResponseBuilder {
        <LinesAttendancePayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinesAttendancePayrollResponseBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    contract_id: Option<String>,
    employee_name: Option<String>,
    gross: Option<String>,
    natura: Option<String>,
    additions: Option<Vec<LinesAttendancePayrollResponseAdditionsItem>>,
    deductions: Option<Vec<LinesAttendancePayrollResponseDeductionsItem>>,
    taxable_base: Option<String>,
    tax_allowance: Option<String>,
    income_tax: Option<String>,
    employee_contributions: Option<String>,
    employer_contributions: Option<String>,
    components: Option<Vec<LinesAttendancePayrollResponseComponentsItem>>,
    net: Option<String>,
    days_worked: Option<String>,
    hours_worked: Option<String>,
    registered_days: Option<String>,
    average_hourly_earnings: Option<String>,
}

impl LinesAttendancePayrollResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn contract_id(mut self, value: impl Into<String>) -> Self {
        self.contract_id = Some(value.into());
        self
    }

    pub fn employee_name(mut self, value: impl Into<String>) -> Self {
        self.employee_name = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn natura(mut self, value: impl Into<String>) -> Self {
        self.natura = Some(value.into());
        self
    }

    pub fn additions(mut self, value: Vec<LinesAttendancePayrollResponseAdditionsItem>) -> Self {
        self.additions = Some(value);
        self
    }

    pub fn deductions(mut self, value: Vec<LinesAttendancePayrollResponseDeductionsItem>) -> Self {
        self.deductions = Some(value);
        self
    }

    pub fn taxable_base(mut self, value: impl Into<String>) -> Self {
        self.taxable_base = Some(value.into());
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

    pub fn components(mut self, value: Vec<LinesAttendancePayrollResponseComponentsItem>) -> Self {
        self.components = Some(value);
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    pub fn days_worked(mut self, value: impl Into<String>) -> Self {
        self.days_worked = Some(value.into());
        self
    }

    pub fn hours_worked(mut self, value: impl Into<String>) -> Self {
        self.hours_worked = Some(value.into());
        self
    }

    pub fn registered_days(mut self, value: impl Into<String>) -> Self {
        self.registered_days = Some(value.into());
        self
    }

    pub fn average_hourly_earnings(mut self, value: impl Into<String>) -> Self {
        self.average_hourly_earnings = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinesAttendancePayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LinesAttendancePayrollResponseBuilder::id)
    /// - [`employee_id`](LinesAttendancePayrollResponseBuilder::employee_id)
    /// - [`employee_name`](LinesAttendancePayrollResponseBuilder::employee_name)
    /// - [`gross`](LinesAttendancePayrollResponseBuilder::gross)
    /// - [`natura`](LinesAttendancePayrollResponseBuilder::natura)
    /// - [`additions`](LinesAttendancePayrollResponseBuilder::additions)
    /// - [`deductions`](LinesAttendancePayrollResponseBuilder::deductions)
    /// - [`taxable_base`](LinesAttendancePayrollResponseBuilder::taxable_base)
    /// - [`tax_allowance`](LinesAttendancePayrollResponseBuilder::tax_allowance)
    /// - [`income_tax`](LinesAttendancePayrollResponseBuilder::income_tax)
    /// - [`employee_contributions`](LinesAttendancePayrollResponseBuilder::employee_contributions)
    /// - [`employer_contributions`](LinesAttendancePayrollResponseBuilder::employer_contributions)
    /// - [`components`](LinesAttendancePayrollResponseBuilder::components)
    /// - [`net`](LinesAttendancePayrollResponseBuilder::net)
    pub fn build(self) -> Result<LinesAttendancePayrollResponse, BuildError> {
        Ok(LinesAttendancePayrollResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            contract_id: self.contract_id,
            employee_name: self
                .employee_name
                .ok_or_else(|| BuildError::missing_field("employee_name"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            natura: self
                .natura
                .ok_or_else(|| BuildError::missing_field("natura"))?,
            additions: self
                .additions
                .ok_or_else(|| BuildError::missing_field("additions"))?,
            deductions: self
                .deductions
                .ok_or_else(|| BuildError::missing_field("deductions"))?,
            taxable_base: self
                .taxable_base
                .ok_or_else(|| BuildError::missing_field("taxable_base"))?,
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
            days_worked: self.days_worked,
            hours_worked: self.hours_worked,
            registered_days: self.registered_days,
            average_hourly_earnings: self.average_hourly_earnings,
        })
    }
}
