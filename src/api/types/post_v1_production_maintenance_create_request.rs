pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1ProductionMaintenanceCreateRequest {
    #[serde(rename = "workCenterId")]
    #[serde(default)]
    pub work_center_id: String,
    pub r#type: PostV1ProductionMaintenanceCreateRequestType,
    #[serde(rename = "plannedDate")]
    #[serde(default)]
    pub planned_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionMaintenanceCreateRequest {
    pub fn builder() -> PostV1ProductionMaintenanceCreateRequestBuilder {
        <PostV1ProductionMaintenanceCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionMaintenanceCreateRequestBuilder {
    work_center_id: Option<String>,
    r#type: Option<PostV1ProductionMaintenanceCreateRequestType>,
    planned_date: Option<String>,
    description: Option<String>,
    notes: Option<String>,
}

impl PostV1ProductionMaintenanceCreateRequestBuilder {
    pub fn work_center_id(mut self, value: impl Into<String>) -> Self {
        self.work_center_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1ProductionMaintenanceCreateRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn planned_date(mut self, value: impl Into<String>) -> Self {
        self.planned_date = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionMaintenanceCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`work_center_id`](PostV1ProductionMaintenanceCreateRequestBuilder::work_center_id)
    /// - [`r#type`](PostV1ProductionMaintenanceCreateRequestBuilder::r#type)
    /// - [`planned_date`](PostV1ProductionMaintenanceCreateRequestBuilder::planned_date)
    pub fn build(self) -> Result<PostV1ProductionMaintenanceCreateRequest, BuildError> {
        Ok(PostV1ProductionMaintenanceCreateRequest {
            work_center_id: self
                .work_center_id
                .ok_or_else(|| BuildError::missing_field("work_center_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            planned_date: self
                .planned_date
                .ok_or_else(|| BuildError::missing_field("planned_date"))?,
            description: self.description,
            notes: self.notes,
        })
    }
}
