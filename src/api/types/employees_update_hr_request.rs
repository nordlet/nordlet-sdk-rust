pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EmployeesUpdateHrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "personalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_code: Option<String>,
    #[serde(rename = "birthDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<EmployeesUpdateHrRequestAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "socialInsuranceNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_insurance_no: Option<String>,
    #[serde(rename = "socialInsuranceStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_insurance_start: Option<NaiveDate>,
    #[serde(rename = "hireDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<NaiveDate>,
    #[serde(rename = "applyAllowance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_allowance: Option<bool>,
    #[serde(rename = "allowanceOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_override: Option<String>,
    #[serde(rename = "pensionAccumulation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pension_accumulation: Option<bool>,
    #[serde(rename = "payrollOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payroll_options: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<EmployeesUpdateHrRequestAttributesItem>>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "terminationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmployeesUpdateHrRequestStatus>,
}

impl EmployeesUpdateHrRequest {
    pub fn builder() -> EmployeesUpdateHrRequestBuilder {
        <EmployeesUpdateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesUpdateHrRequestBuilder {
    code: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    personal_code: Option<String>,
    birth_date: Option<NaiveDate>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<EmployeesUpdateHrRequestAddress>,
    iban: Option<String>,
    social_insurance_no: Option<String>,
    social_insurance_start: Option<NaiveDate>,
    hire_date: Option<NaiveDate>,
    apply_allowance: Option<bool>,
    allowance_override: Option<String>,
    pension_accumulation: Option<bool>,
    payroll_options: Option<HashMap<String, String>>,
    notes: Option<String>,
    attributes: Option<Vec<EmployeesUpdateHrRequestAttributesItem>>,
    id: Option<String>,
    termination_date: Option<NaiveDate>,
    status: Option<EmployeesUpdateHrRequestStatus>,
}

impl EmployeesUpdateHrRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn personal_code(mut self, value: impl Into<String>) -> Self {
        self.personal_code = Some(value.into());
        self
    }

    pub fn birth_date(mut self, value: NaiveDate) -> Self {
        self.birth_date = Some(value);
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

    pub fn address(mut self, value: EmployeesUpdateHrRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn iban(mut self, value: impl Into<String>) -> Self {
        self.iban = Some(value.into());
        self
    }

    pub fn social_insurance_no(mut self, value: impl Into<String>) -> Self {
        self.social_insurance_no = Some(value.into());
        self
    }

    pub fn social_insurance_start(mut self, value: NaiveDate) -> Self {
        self.social_insurance_start = Some(value);
        self
    }

    pub fn hire_date(mut self, value: NaiveDate) -> Self {
        self.hire_date = Some(value);
        self
    }

    pub fn apply_allowance(mut self, value: bool) -> Self {
        self.apply_allowance = Some(value);
        self
    }

    pub fn allowance_override(mut self, value: impl Into<String>) -> Self {
        self.allowance_override = Some(value.into());
        self
    }

    pub fn pension_accumulation(mut self, value: bool) -> Self {
        self.pension_accumulation = Some(value);
        self
    }

    pub fn payroll_options(mut self, value: HashMap<String, String>) -> Self {
        self.payroll_options = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: Vec<EmployeesUpdateHrRequestAttributesItem>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn termination_date(mut self, value: NaiveDate) -> Self {
        self.termination_date = Some(value);
        self
    }

    pub fn status(mut self, value: EmployeesUpdateHrRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesUpdateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesUpdateHrRequestBuilder::id)
    pub fn build(self) -> Result<EmployeesUpdateHrRequest, BuildError> {
        Ok(EmployeesUpdateHrRequest {
            code: self.code,
            first_name: self.first_name,
            last_name: self.last_name,
            personal_code: self.personal_code,
            birth_date: self.birth_date,
            email: self.email,
            phone: self.phone,
            address: self.address,
            iban: self.iban,
            social_insurance_no: self.social_insurance_no,
            social_insurance_start: self.social_insurance_start,
            hire_date: self.hire_date,
            apply_allowance: self.apply_allowance,
            allowance_override: self.allowance_override,
            pension_accumulation: self.pension_accumulation,
            payroll_options: self.payroll_options,
            notes: self.notes,
            attributes: self.attributes,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            termination_date: self.termination_date,
            status: self.status,
        })
    }
}
