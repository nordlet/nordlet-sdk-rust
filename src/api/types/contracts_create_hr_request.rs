pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContractsCreateHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "positionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(rename = "departmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(rename = "scheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(rename = "agreementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "contractNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ContractsCreateHrRequestType>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "baseSalary")]
    #[serde(default)]
    pub base_salary: String,
    #[serde(rename = "salaryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_type: Option<ContractsCreateHrRequestSalaryType>,
    #[serde(rename = "workHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ContractsCreateHrRequest {
    pub fn builder() -> ContractsCreateHrRequestBuilder {
        <ContractsCreateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractsCreateHrRequestBuilder {
    employee_id: Option<String>,
    position_id: Option<String>,
    department_id: Option<String>,
    schedule_id: Option<String>,
    agreement_id: Option<String>,
    contract_no: Option<String>,
    r#type: Option<ContractsCreateHrRequestType>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    base_salary: Option<String>,
    salary_type: Option<ContractsCreateHrRequestSalaryType>,
    work_hours: Option<String>,
    notes: Option<String>,
}

impl ContractsCreateHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn position_id(mut self, value: impl Into<String>) -> Self {
        self.position_id = Some(value.into());
        self
    }

    pub fn department_id(mut self, value: impl Into<String>) -> Self {
        self.department_id = Some(value.into());
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn contract_no(mut self, value: impl Into<String>) -> Self {
        self.contract_no = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ContractsCreateHrRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn base_salary(mut self, value: impl Into<String>) -> Self {
        self.base_salary = Some(value.into());
        self
    }

    pub fn salary_type(mut self, value: ContractsCreateHrRequestSalaryType) -> Self {
        self.salary_type = Some(value);
        self
    }

    pub fn work_hours(mut self, value: impl Into<String>) -> Self {
        self.work_hours = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContractsCreateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](ContractsCreateHrRequestBuilder::employee_id)
    /// - [`start_date`](ContractsCreateHrRequestBuilder::start_date)
    /// - [`base_salary`](ContractsCreateHrRequestBuilder::base_salary)
    pub fn build(self) -> Result<ContractsCreateHrRequest, BuildError> {
        Ok(ContractsCreateHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            position_id: self.position_id,
            department_id: self.department_id,
            schedule_id: self.schedule_id,
            agreement_id: self.agreement_id,
            contract_no: self.contract_no,
            r#type: self.r#type,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            base_salary: self
                .base_salary
                .ok_or_else(|| BuildError::missing_field("base_salary"))?,
            salary_type: self.salary_type,
            work_hours: self.work_hours,
            notes: self.notes,
        })
    }
}
