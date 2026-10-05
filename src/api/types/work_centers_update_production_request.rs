pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkCentersUpdateProductionRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "costPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_hour: Option<String>,
    #[serde(rename = "costAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_account_code: Option<String>,
    #[serde(rename = "maintenanceIntervalDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_interval_days: Option<i64>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl WorkCentersUpdateProductionRequest {
    pub fn builder() -> WorkCentersUpdateProductionRequestBuilder {
        <WorkCentersUpdateProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkCentersUpdateProductionRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    cost_per_hour: Option<String>,
    cost_account_code: Option<String>,
    maintenance_interval_days: Option<i64>,
    is_active: Option<bool>,
    notes: Option<String>,
}

impl WorkCentersUpdateProductionRequestBuilder {
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

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkCentersUpdateProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkCentersUpdateProductionRequestBuilder::id)
    pub fn build(self) -> Result<WorkCentersUpdateProductionRequest, BuildError> {
        Ok(WorkCentersUpdateProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            cost_per_hour: self.cost_per_hour,
            cost_account_code: self.cost_account_code,
            maintenance_interval_days: self.maintenance_interval_days,
            is_active: self.is_active,
            notes: self.notes,
        })
    }
}
