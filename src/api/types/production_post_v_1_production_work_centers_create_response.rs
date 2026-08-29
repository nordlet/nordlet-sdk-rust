pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionWorkCentersCreateResponse {
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
    pub next_maintenance_date: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1ProductionWorkCentersCreateResponse {
    pub fn builder() -> PostV1ProductionWorkCentersCreateResponseBuilder {
        <PostV1ProductionWorkCentersCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionWorkCentersCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    cost_per_hour: Option<String>,
    cost_account_code: Option<String>,
    maintenance_interval_days: Option<i64>,
    next_maintenance_date: Option<String>,
    is_active: Option<bool>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1ProductionWorkCentersCreateResponseBuilder {
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

    pub fn next_maintenance_date(mut self, value: impl Into<String>) -> Self {
        self.next_maintenance_date = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionWorkCentersCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionWorkCentersCreateResponseBuilder::id)
    /// - [`code`](PostV1ProductionWorkCentersCreateResponseBuilder::code)
    /// - [`name`](PostV1ProductionWorkCentersCreateResponseBuilder::name)
    /// - [`cost_per_hour`](PostV1ProductionWorkCentersCreateResponseBuilder::cost_per_hour)
    /// - [`is_active`](PostV1ProductionWorkCentersCreateResponseBuilder::is_active)
    /// - [`created_at`](PostV1ProductionWorkCentersCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1ProductionWorkCentersCreateResponse, BuildError> {
        Ok(PostV1ProductionWorkCentersCreateResponse {
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
