pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MaintenanceCreateProductionRequest {
    #[serde(rename = "workCenterId")]
    #[serde(default)]
    pub work_center_id: String,
    pub r#type: MaintenanceCreateProductionRequestType,
    #[serde(rename = "plannedDate")]
    #[serde(default)]
    pub planned_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl MaintenanceCreateProductionRequest {
    pub fn builder() -> MaintenanceCreateProductionRequestBuilder {
        <MaintenanceCreateProductionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MaintenanceCreateProductionRequestBuilder {
    work_center_id: Option<String>,
    r#type: Option<MaintenanceCreateProductionRequestType>,
    planned_date: Option<NaiveDate>,
    description: Option<String>,
    notes: Option<String>,
}

impl MaintenanceCreateProductionRequestBuilder {
    pub fn work_center_id(mut self, value: impl Into<String>) -> Self {
        self.work_center_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: MaintenanceCreateProductionRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn planned_date(mut self, value: NaiveDate) -> Self {
        self.planned_date = Some(value);
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

    /// Consumes the builder and constructs a [`MaintenanceCreateProductionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`work_center_id`](MaintenanceCreateProductionRequestBuilder::work_center_id)
    /// - [`r#type`](MaintenanceCreateProductionRequestBuilder::r#type)
    /// - [`planned_date`](MaintenanceCreateProductionRequestBuilder::planned_date)
    pub fn build(self) -> Result<MaintenanceCreateProductionRequest, BuildError> {
        Ok(MaintenanceCreateProductionRequest {
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
