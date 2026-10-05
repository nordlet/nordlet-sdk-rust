pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MaintenanceCreateProductionResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "workCenterId")]
    #[serde(default)]
    pub work_center_id: String,
    pub r#type: MaintenanceCreateProductionResponseType,
    pub status: MaintenanceCreateProductionResponseStatus,
    #[serde(rename = "plannedDate")]
    #[serde(default)]
    pub planned_date: NaiveDate,
    #[serde(rename = "completedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "downtimeHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downtime_hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl MaintenanceCreateProductionResponse {
    pub fn builder() -> MaintenanceCreateProductionResponseBuilder {
        <MaintenanceCreateProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceCreateProductionResponseBuilder {
    id: Option<String>,
    work_center_id: Option<String>,
    r#type: Option<MaintenanceCreateProductionResponseType>,
    status: Option<MaintenanceCreateProductionResponseStatus>,
    planned_date: Option<NaiveDate>,
    completed_date: Option<NaiveDate>,
    description: Option<String>,
    downtime_hours: Option<String>,
    cost: Option<String>,
    notes: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl MaintenanceCreateProductionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn work_center_id(mut self, value: impl Into<String>) -> Self {
        self.work_center_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: MaintenanceCreateProductionResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: MaintenanceCreateProductionResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn planned_date(mut self, value: NaiveDate) -> Self {
        self.planned_date = Some(value);
        self
    }

    pub fn completed_date(mut self, value: NaiveDate) -> Self {
        self.completed_date = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn downtime_hours(mut self, value: impl Into<String>) -> Self {
        self.downtime_hours = Some(value.into());
        self
    }

    pub fn cost(mut self, value: impl Into<String>) -> Self {
        self.cost = Some(value.into());
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

    /// Consumes the builder and constructs a [`MaintenanceCreateProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MaintenanceCreateProductionResponseBuilder::id)
    /// - [`work_center_id`](MaintenanceCreateProductionResponseBuilder::work_center_id)
    /// - [`r#type`](MaintenanceCreateProductionResponseBuilder::r#type)
    /// - [`status`](MaintenanceCreateProductionResponseBuilder::status)
    /// - [`planned_date`](MaintenanceCreateProductionResponseBuilder::planned_date)
    /// - [`created_at`](MaintenanceCreateProductionResponseBuilder::created_at)
    pub fn build(self) -> Result<MaintenanceCreateProductionResponse, BuildError> {
        Ok(MaintenanceCreateProductionResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            work_center_id: self
                .work_center_id
                .ok_or_else(|| BuildError::missing_field("work_center_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            planned_date: self
                .planned_date
                .ok_or_else(|| BuildError::missing_field("planned_date"))?,
            completed_date: self.completed_date,
            description: self.description,
            downtime_hours: self.downtime_hours,
            cost: self.cost,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
