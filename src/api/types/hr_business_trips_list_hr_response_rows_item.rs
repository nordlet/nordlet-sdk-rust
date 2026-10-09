pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BusinessTripsListHrResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "destinationCountryCode")]
    #[serde(default)]
    pub destination_country_code: String,
    #[serde(default)]
    pub purpose: String,
    #[serde(rename = "startDate")]
    #[serde(default)]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    #[serde(default)]
    pub end_date: NaiveDate,
    #[serde(default)]
    pub days: i64,
    #[serde(rename = "dailyRate")]
    #[serde(default)]
    pub daily_rate: String,
    #[serde(rename = "perDiemAmount")]
    #[serde(default)]
    pub per_diem_amount: String,
    pub status: BusinessTripsListHrResponseRowsItemStatus,
    #[serde(rename = "payrollRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payroll_run_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl BusinessTripsListHrResponseRowsItem {
    pub fn builder() -> BusinessTripsListHrResponseRowsItemBuilder {
        <BusinessTripsListHrResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsListHrResponseRowsItemBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    destination_country_code: Option<String>,
    purpose: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    days: Option<i64>,
    daily_rate: Option<String>,
    per_diem_amount: Option<String>,
    status: Option<BusinessTripsListHrResponseRowsItemStatus>,
    payroll_run_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl BusinessTripsListHrResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn destination_country_code(mut self, value: impl Into<String>) -> Self {
        self.destination_country_code = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
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

    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    pub fn daily_rate(mut self, value: impl Into<String>) -> Self {
        self.daily_rate = Some(value.into());
        self
    }

    pub fn per_diem_amount(mut self, value: impl Into<String>) -> Self {
        self.per_diem_amount = Some(value.into());
        self
    }

    pub fn status(mut self, value: BusinessTripsListHrResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payroll_run_id(mut self, value: impl Into<String>) -> Self {
        self.payroll_run_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsListHrResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BusinessTripsListHrResponseRowsItemBuilder::id)
    /// - [`employee_id`](BusinessTripsListHrResponseRowsItemBuilder::employee_id)
    /// - [`destination_country_code`](BusinessTripsListHrResponseRowsItemBuilder::destination_country_code)
    /// - [`purpose`](BusinessTripsListHrResponseRowsItemBuilder::purpose)
    /// - [`start_date`](BusinessTripsListHrResponseRowsItemBuilder::start_date)
    /// - [`end_date`](BusinessTripsListHrResponseRowsItemBuilder::end_date)
    /// - [`days`](BusinessTripsListHrResponseRowsItemBuilder::days)
    /// - [`daily_rate`](BusinessTripsListHrResponseRowsItemBuilder::daily_rate)
    /// - [`per_diem_amount`](BusinessTripsListHrResponseRowsItemBuilder::per_diem_amount)
    /// - [`status`](BusinessTripsListHrResponseRowsItemBuilder::status)
    /// - [`created_at`](BusinessTripsListHrResponseRowsItemBuilder::created_at)
    /// - [`updated_at`](BusinessTripsListHrResponseRowsItemBuilder::updated_at)
    pub fn build(self) -> Result<BusinessTripsListHrResponseRowsItem, BuildError> {
        Ok(BusinessTripsListHrResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            destination_country_code: self
                .destination_country_code
                .ok_or_else(|| BuildError::missing_field("destination_country_code"))?,
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
            daily_rate: self
                .daily_rate
                .ok_or_else(|| BuildError::missing_field("daily_rate"))?,
            per_diem_amount: self
                .per_diem_amount
                .ok_or_else(|| BuildError::missing_field("per_diem_amount"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            payroll_run_id: self.payroll_run_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
