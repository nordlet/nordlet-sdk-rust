pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EmployeesRecordsCreateHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    pub r#type: EmployeesRecordsCreateHrRequestType,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<NaiveDate>,
    #[serde(rename = "validUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<NaiveDate>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl EmployeesRecordsCreateHrRequest {
    pub fn builder() -> EmployeesRecordsCreateHrRequestBuilder {
        <EmployeesRecordsCreateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesRecordsCreateHrRequestBuilder {
    employee_id: Option<String>,
    r#type: Option<EmployeesRecordsCreateHrRequestType>,
    title: Option<String>,
    institution: Option<String>,
    issued_at: Option<NaiveDate>,
    valid_until: Option<NaiveDate>,
    file_id: Option<String>,
    notes: Option<String>,
}

impl EmployeesRecordsCreateHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: EmployeesRecordsCreateHrRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn institution(mut self, value: impl Into<String>) -> Self {
        self.institution = Some(value.into());
        self
    }

    pub fn issued_at(mut self, value: NaiveDate) -> Self {
        self.issued_at = Some(value);
        self
    }

    pub fn valid_until(mut self, value: NaiveDate) -> Self {
        self.valid_until = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesRecordsCreateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](EmployeesRecordsCreateHrRequestBuilder::employee_id)
    /// - [`r#type`](EmployeesRecordsCreateHrRequestBuilder::r#type)
    /// - [`title`](EmployeesRecordsCreateHrRequestBuilder::title)
    pub fn build(self) -> Result<EmployeesRecordsCreateHrRequest, BuildError> {
        Ok(EmployeesRecordsCreateHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            institution: self.institution,
            issued_at: self.issued_at,
            valid_until: self.valid_until,
            file_id: self.file_id,
            notes: self.notes,
        })
    }
}
