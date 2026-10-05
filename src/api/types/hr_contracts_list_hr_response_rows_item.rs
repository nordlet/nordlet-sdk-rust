pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContractsListHrResponseRowsItem {
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
    #[serde(rename = "agreementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(rename = "contractNo")]
    #[serde(default)]
    pub contract_no: String,
    pub r#type: ContractsListHrResponseRowsItemType,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "endReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
    #[serde(rename = "baseSalary")]
    #[serde(default)]
    pub base_salary: String,
    #[serde(rename = "salaryType")]
    pub salary_type: ContractsListHrResponseRowsItemSalaryType,
    #[serde(rename = "workHours")]
    #[serde(default)]
    pub work_hours: String,
    #[serde(rename = "workHoursUnit")]
    pub work_hours_unit: ContractsListHrResponseRowsItemWorkHoursUnit,
    pub status: ContractsListHrResponseRowsItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ContractsListHrResponseRowsItem {
    pub fn builder() -> ContractsListHrResponseRowsItemBuilder {
        <ContractsListHrResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractsListHrResponseRowsItemBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    position_id: Option<String>,
    department_id: Option<String>,
    schedule_id: Option<String>,
    agreement_id: Option<String>,
    contract_no: Option<String>,
    r#type: Option<ContractsListHrResponseRowsItemType>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    end_reason: Option<String>,
    base_salary: Option<String>,
    salary_type: Option<ContractsListHrResponseRowsItemSalaryType>,
    work_hours: Option<String>,
    work_hours_unit: Option<ContractsListHrResponseRowsItemWorkHoursUnit>,
    status: Option<ContractsListHrResponseRowsItemStatus>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ContractsListHrResponseRowsItemBuilder {
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

    pub fn agreement_id(mut self, value: impl Into<String>) -> Self {
        self.agreement_id = Some(value.into());
        self
    }

    pub fn contract_no(mut self, value: impl Into<String>) -> Self {
        self.contract_no = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ContractsListHrResponseRowsItemType) -> Self {
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

    pub fn end_reason(mut self, value: impl Into<String>) -> Self {
        self.end_reason = Some(value.into());
        self
    }

    pub fn base_salary(mut self, value: impl Into<String>) -> Self {
        self.base_salary = Some(value.into());
        self
    }

    pub fn salary_type(mut self, value: ContractsListHrResponseRowsItemSalaryType) -> Self {
        self.salary_type = Some(value);
        self
    }

    pub fn work_hours(mut self, value: impl Into<String>) -> Self {
        self.work_hours = Some(value.into());
        self
    }

    pub fn work_hours_unit(mut self, value: ContractsListHrResponseRowsItemWorkHoursUnit) -> Self {
        self.work_hours_unit = Some(value);
        self
    }

    pub fn status(mut self, value: ContractsListHrResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContractsListHrResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ContractsListHrResponseRowsItemBuilder::id)
    /// - [`employee_id`](ContractsListHrResponseRowsItemBuilder::employee_id)
    /// - [`contract_no`](ContractsListHrResponseRowsItemBuilder::contract_no)
    /// - [`r#type`](ContractsListHrResponseRowsItemBuilder::r#type)
    /// - [`start_date`](ContractsListHrResponseRowsItemBuilder::start_date)
    /// - [`base_salary`](ContractsListHrResponseRowsItemBuilder::base_salary)
    /// - [`salary_type`](ContractsListHrResponseRowsItemBuilder::salary_type)
    /// - [`work_hours`](ContractsListHrResponseRowsItemBuilder::work_hours)
    /// - [`work_hours_unit`](ContractsListHrResponseRowsItemBuilder::work_hours_unit)
    /// - [`status`](ContractsListHrResponseRowsItemBuilder::status)
    /// - [`created_at`](ContractsListHrResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<ContractsListHrResponseRowsItem, BuildError> {
        Ok(ContractsListHrResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            position_id: self.position_id,
            department_id: self.department_id,
            schedule_id: self.schedule_id,
            agreement_id: self.agreement_id,
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
            work_hours: self
                .work_hours
                .ok_or_else(|| BuildError::missing_field("work_hours"))?,
            work_hours_unit: self
                .work_hours_unit
                .ok_or_else(|| BuildError::missing_field("work_hours_unit"))?,
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
