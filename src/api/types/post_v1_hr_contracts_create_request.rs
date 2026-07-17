pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrContractsCreateRequest {
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
    #[serde(rename = "contractNo")]
    #[serde(default)]
    pub contract_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostV1HrContractsCreateRequestType>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "baseSalary")]
    #[serde(default)]
    pub base_salary: String,
    #[serde(rename = "salaryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_type: Option<PostV1HrContractsCreateRequestSalaryType>,
    #[serde(rename = "workHoursPerWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours_per_week: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1HrContractsCreateRequest {
    pub fn builder() -> PostV1HrContractsCreateRequestBuilder {
        <PostV1HrContractsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrContractsCreateRequestBuilder {
    employee_id: Option<String>,
    position_id: Option<String>,
    department_id: Option<String>,
    schedule_id: Option<String>,
    contract_no: Option<String>,
    r#type: Option<PostV1HrContractsCreateRequestType>,
    start_date: Option<String>,
    end_date: Option<String>,
    base_salary: Option<String>,
    salary_type: Option<PostV1HrContractsCreateRequestSalaryType>,
    work_hours_per_week: Option<String>,
    notes: Option<String>,
}

impl PostV1HrContractsCreateRequestBuilder {
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

    pub fn contract_no(mut self, value: impl Into<String>) -> Self {
        self.contract_no = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1HrContractsCreateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn base_salary(mut self, value: impl Into<String>) -> Self {
        self.base_salary = Some(value.into());
        self
    }

    pub fn salary_type(mut self, value: PostV1HrContractsCreateRequestSalaryType) -> Self {
        self.salary_type = Some(value);
        self
    }

    pub fn work_hours_per_week(mut self, value: impl Into<String>) -> Self {
        self.work_hours_per_week = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrContractsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1HrContractsCreateRequestBuilder::employee_id)
    /// - [`contract_no`](PostV1HrContractsCreateRequestBuilder::contract_no)
    /// - [`start_date`](PostV1HrContractsCreateRequestBuilder::start_date)
    /// - [`base_salary`](PostV1HrContractsCreateRequestBuilder::base_salary)
    pub fn build(self) -> Result<PostV1HrContractsCreateRequest, BuildError> {
        Ok(PostV1HrContractsCreateRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            position_id: self.position_id,
            department_id: self.department_id,
            schedule_id: self.schedule_id,
            contract_no: self
                .contract_no
                .ok_or_else(|| BuildError::missing_field("contract_no"))?,
            r#type: self.r#type,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            base_salary: self
                .base_salary
                .ok_or_else(|| BuildError::missing_field("base_salary"))?,
            salary_type: self.salary_type,
            work_hours_per_week: self.work_hours_per_week,
            notes: self.notes,
        })
    }
}
