pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProductionMaintenanceCompleteResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "workCenterId")]
    #[serde(default)]
    pub work_center_id: String,
    pub r#type: PostV1ProductionMaintenanceCompleteResponseType,
    pub status: PostV1ProductionMaintenanceCompleteResponseStatus,
    #[serde(rename = "plannedDate")]
    #[serde(default)]
    pub planned_date: String,
    #[serde(rename = "completedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_date: Option<String>,
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
    pub created_at: String,
}

impl PostV1ProductionMaintenanceCompleteResponse {
    pub fn builder() -> PostV1ProductionMaintenanceCompleteResponseBuilder {
        <PostV1ProductionMaintenanceCompleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionMaintenanceCompleteResponseBuilder {
    id: Option<String>,
    work_center_id: Option<String>,
    r#type: Option<PostV1ProductionMaintenanceCompleteResponseType>,
    status: Option<PostV1ProductionMaintenanceCompleteResponseStatus>,
    planned_date: Option<String>,
    completed_date: Option<String>,
    description: Option<String>,
    downtime_hours: Option<String>,
    cost: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1ProductionMaintenanceCompleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn work_center_id(mut self, value: impl Into<String>) -> Self {
        self.work_center_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1ProductionMaintenanceCompleteResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1ProductionMaintenanceCompleteResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn planned_date(mut self, value: impl Into<String>) -> Self {
        self.planned_date = Some(value.into());
        self
    }

    pub fn completed_date(mut self, value: impl Into<String>) -> Self {
        self.completed_date = Some(value.into());
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionMaintenanceCompleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionMaintenanceCompleteResponseBuilder::id)
    /// - [`work_center_id`](PostV1ProductionMaintenanceCompleteResponseBuilder::work_center_id)
    /// - [`r#type`](PostV1ProductionMaintenanceCompleteResponseBuilder::r#type)
    /// - [`status`](PostV1ProductionMaintenanceCompleteResponseBuilder::status)
    /// - [`planned_date`](PostV1ProductionMaintenanceCompleteResponseBuilder::planned_date)
    /// - [`created_at`](PostV1ProductionMaintenanceCompleteResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1ProductionMaintenanceCompleteResponse, BuildError> {
        Ok(PostV1ProductionMaintenanceCompleteResponse {
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
