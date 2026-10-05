pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MaintenanceCompleteProductionRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "completedDate")]
    #[serde(default)]
    pub completed_date: NaiveDate,
    #[serde(rename = "downtimeHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downtime_hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl MaintenanceCompleteProductionRequest {
    pub fn builder() -> MaintenanceCompleteProductionRequestBuilder {
        <MaintenanceCompleteProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceCompleteProductionRequestBuilder {
    id: Option<String>,
    completed_date: Option<NaiveDate>,
    downtime_hours: Option<String>,
    cost: Option<String>,
    notes: Option<String>,
}

impl MaintenanceCompleteProductionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn completed_date(mut self, value: NaiveDate) -> Self {
        self.completed_date = Some(value);
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

    /// Consumes the builder and constructs a [`MaintenanceCompleteProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MaintenanceCompleteProductionRequestBuilder::id)
    /// - [`completed_date`](MaintenanceCompleteProductionRequestBuilder::completed_date)
    pub fn build(self) -> Result<MaintenanceCompleteProductionRequest, BuildError> {
        Ok(MaintenanceCompleteProductionRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            completed_date: self
                .completed_date
                .ok_or_else(|| BuildError::missing_field("completed_date"))?,
            downtime_hours: self.downtime_hours,
            cost: self.cost,
            notes: self.notes,
        })
    }
}
