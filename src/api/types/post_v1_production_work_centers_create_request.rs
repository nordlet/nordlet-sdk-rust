pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionWorkCentersCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "costPerHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_hour: Option<String>,
    #[serde(rename = "costAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_account_code: Option<String>,
    #[serde(rename = "maintenanceIntervalDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_interval_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionWorkCentersCreateRequest {
    pub fn builder() -> PostV1ProductionWorkCentersCreateRequestBuilder {
        <PostV1ProductionWorkCentersCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionWorkCentersCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    cost_per_hour: Option<String>,
    cost_account_code: Option<String>,
    maintenance_interval_days: Option<i64>,
    notes: Option<String>,
}

impl PostV1ProductionWorkCentersCreateRequestBuilder {
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionWorkCentersCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1ProductionWorkCentersCreateRequestBuilder::code)
    /// - [`name`](PostV1ProductionWorkCentersCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1ProductionWorkCentersCreateRequest, BuildError> {
        Ok(PostV1ProductionWorkCentersCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            cost_per_hour: self.cost_per_hour,
            cost_account_code: self.cost_account_code,
            maintenance_interval_days: self.maintenance_interval_days,
            notes: self.notes,
        })
    }
}
