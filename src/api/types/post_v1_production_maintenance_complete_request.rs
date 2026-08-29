pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionMaintenanceCompleteRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "completedDate")]
    #[serde(default)]
    pub completed_date: String,
    #[serde(rename = "downtimeHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downtime_hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionMaintenanceCompleteRequest {
    pub fn builder() -> PostV1ProductionMaintenanceCompleteRequestBuilder {
        <PostV1ProductionMaintenanceCompleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionMaintenanceCompleteRequestBuilder {
    id: Option<String>,
    completed_date: Option<String>,
    downtime_hours: Option<String>,
    cost: Option<String>,
    notes: Option<String>,
}

impl PostV1ProductionMaintenanceCompleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn completed_date(mut self, value: impl Into<String>) -> Self {
        self.completed_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ProductionMaintenanceCompleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionMaintenanceCompleteRequestBuilder::id)
    /// - [`completed_date`](PostV1ProductionMaintenanceCompleteRequestBuilder::completed_date)
    pub fn build(self) -> Result<PostV1ProductionMaintenanceCompleteRequest, BuildError> {
        Ok(PostV1ProductionMaintenanceCompleteRequest {
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
