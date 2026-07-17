pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrContractsEndResponse {
    #[serde(default)]
    pub id: String,
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
    pub r#type: PostV1HrContractsEndResponseType,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: String,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "endReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
    #[serde(rename = "baseSalary")]
    #[serde(default)]
    pub base_salary: String,
    #[serde(rename = "salaryType")]
    pub salary_type: PostV1HrContractsEndResponseSalaryType,
    #[serde(rename = "workHoursPerWeek")]
    #[serde(default)]
    pub work_hours_per_week: String,
    pub status: PostV1HrContractsEndResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1HrContractsEndResponse {
    pub fn builder() -> PostV1HrContractsEndResponseBuilder {
        <PostV1HrContractsEndResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrContractsEndResponseBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    position_id: Option<String>,
    department_id: Option<String>,
    schedule_id: Option<String>,
    contract_no: Option<String>,
    r#type: Option<PostV1HrContractsEndResponseType>,
    start_date: Option<String>,
    end_date: Option<String>,
    end_reason: Option<String>,
    base_salary: Option<String>,
    salary_type: Option<PostV1HrContractsEndResponseSalaryType>,
    work_hours_per_week: Option<String>,
    status: Option<PostV1HrContractsEndResponseStatus>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1HrContractsEndResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn r#type(mut self, value: PostV1HrContractsEndResponseType) -> Self {
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

    pub fn end_reason(mut self, value: impl Into<String>) -> Self {
        self.end_reason = Some(value.into());
        self
    }

    pub fn base_salary(mut self, value: impl Into<String>) -> Self {
        self.base_salary = Some(value.into());
        self
    }

    pub fn salary_type(mut self, value: PostV1HrContractsEndResponseSalaryType) -> Self {
        self.salary_type = Some(value);
        self
    }

    pub fn work_hours_per_week(mut self, value: impl Into<String>) -> Self {
        self.work_hours_per_week = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1HrContractsEndResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrContractsEndResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrContractsEndResponseBuilder::id)
    /// - [`employee_id`](PostV1HrContractsEndResponseBuilder::employee_id)
    /// - [`contract_no`](PostV1HrContractsEndResponseBuilder::contract_no)
    /// - [`r#type`](PostV1HrContractsEndResponseBuilder::r#type)
    /// - [`start_date`](PostV1HrContractsEndResponseBuilder::start_date)
    /// - [`base_salary`](PostV1HrContractsEndResponseBuilder::base_salary)
    /// - [`salary_type`](PostV1HrContractsEndResponseBuilder::salary_type)
    /// - [`work_hours_per_week`](PostV1HrContractsEndResponseBuilder::work_hours_per_week)
    /// - [`status`](PostV1HrContractsEndResponseBuilder::status)
    /// - [`created_at`](PostV1HrContractsEndResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1HrContractsEndResponse, BuildError> {
        Ok(PostV1HrContractsEndResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            position_id: self.position_id,
            department_id: self.department_id,
            schedule_id: self.schedule_id,
            contract_no: self
                .contract_no
                .ok_or_else(|| BuildError::missing_field("contract_no"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date,
            end_reason: self.end_reason,
            base_salary: self
                .base_salary
                .ok_or_else(|| BuildError::missing_field("base_salary"))?,
            salary_type: self
                .salary_type
                .ok_or_else(|| BuildError::missing_field("salary_type"))?,
            work_hours_per_week: self
                .work_hours_per_week
                .ok_or_else(|| BuildError::missing_field("work_hours_per_week"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
