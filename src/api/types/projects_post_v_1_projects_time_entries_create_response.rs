pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub hours: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub billable: bool,
    #[serde(rename = "hourlyRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
    #[serde(rename = "billedInvoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billed_invoice_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl PostV1ProjectsTimeEntriesCreateResponse {
    pub fn builder() -> PostV1ProjectsTimeEntriesCreateResponseBuilder {
        <PostV1ProjectsTimeEntriesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesCreateResponseBuilder {
    id: Option<String>,
    project_id: Option<String>,
    employee_id: Option<String>,
    date: Option<String>,
    hours: Option<String>,
    description: Option<String>,
    billable: Option<bool>,
    hourly_rate: Option<String>,
    billed_invoice_id: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl PostV1ProjectsTimeEntriesCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn billable(mut self, value: bool) -> Self {
        self.billable = Some(value);
        self
    }

    pub fn hourly_rate(mut self, value: impl Into<String>) -> Self {
        self.hourly_rate = Some(value.into());
        self
    }

    pub fn billed_invoice_id(mut self, value: impl Into<String>) -> Self {
        self.billed_invoice_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProjectsTimeEntriesCreateResponseBuilder::id)
    /// - [`project_id`](PostV1ProjectsTimeEntriesCreateResponseBuilder::project_id)
    /// - [`date`](PostV1ProjectsTimeEntriesCreateResponseBuilder::date)
    /// - [`hours`](PostV1ProjectsTimeEntriesCreateResponseBuilder::hours)
    /// - [`billable`](PostV1ProjectsTimeEntriesCreateResponseBuilder::billable)
    /// - [`created_at`](PostV1ProjectsTimeEntriesCreateResponseBuilder::created_at)
    /// - [`updated_at`](PostV1ProjectsTimeEntriesCreateResponseBuilder::updated_at)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesCreateResponse, BuildError> {
        Ok(PostV1ProjectsTimeEntriesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            project_id: self
                .project_id
                .ok_or_else(|| BuildError::missing_field("project_id"))?,
            employee_id: self.employee_id,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            hours: self
                .hours
                .ok_or_else(|| BuildError::missing_field("hours"))?,
            description: self.description,
            billable: self
                .billable
                .ok_or_else(|| BuildError::missing_field("billable"))?,
            hourly_rate: self.hourly_rate,
            billed_invoice_id: self.billed_invoice_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
