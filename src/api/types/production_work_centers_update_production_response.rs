pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkCentersUpdateProductionResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "costPerHour")]
    #[serde(default)]
    pub cost_per_hour: String,
    #[serde(rename = "costAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_account_code: Option<String>,
    #[serde(rename = "maintenanceIntervalDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_interval_days: Option<i64>,
    #[serde(rename = "nextMaintenanceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_date: Option<NaiveDate>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl WorkCentersUpdateProductionResponse {
    pub fn builder() -> WorkCentersUpdateProductionResponseBuilder {
        <WorkCentersUpdateProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkCentersUpdateProductionResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    cost_per_hour: Option<String>,
    cost_account_code: Option<String>,
    maintenance_interval_days: Option<i64>,
    next_maintenance_date: Option<NaiveDate>,
    is_active: Option<bool>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl WorkCentersUpdateProductionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn cost_per_hour(mut self, value: impl Into<String>) -> Self {
        self.cost_per_hour = Some(value.into());
        self
    }

    pub fn cost_account_code(mut self, value: impl Into<String>) -> Self {
        self.cost_account_code = Some(value.into());
        self
    }

    pub fn maintenance_interval_days(mut self, value: i64) -> Self {
        self.maintenance_interval_days = Some(value);
        self
    }

    pub fn next_maintenance_date(mut self, value: NaiveDate) -> Self {
        self.next_maintenance_date = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
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

    /// Consumes the builder and constructs a [`WorkCentersUpdateProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkCentersUpdateProductionResponseBuilder::id)
    /// - [`code`](WorkCentersUpdateProductionResponseBuilder::code)
    /// - [`name`](WorkCentersUpdateProductionResponseBuilder::name)
    /// - [`cost_per_hour`](WorkCentersUpdateProductionResponseBuilder::cost_per_hour)
    /// - [`is_active`](WorkCentersUpdateProductionResponseBuilder::is_active)
    /// - [`created_at`](WorkCentersUpdateProductionResponseBuilder::created_at)
    pub fn build(self) -> Result<WorkCentersUpdateProductionResponse, BuildError> {
        Ok(WorkCentersUpdateProductionResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            cost_per_hour: self
                .cost_per_hour
                .ok_or_else(|| BuildError::missing_field("cost_per_hour"))?,
            cost_account_code: self.cost_account_code,
            maintenance_interval_days: self.maintenance_interval_days,
            next_maintenance_date: self.next_maintenance_date,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
