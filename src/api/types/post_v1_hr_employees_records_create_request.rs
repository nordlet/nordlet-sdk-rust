pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesRecordsCreateRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    pub r#type: PostV1HrEmployeesRecordsCreateRequestType,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<String>,
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(rename = "validUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1HrEmployeesRecordsCreateRequest {
    pub fn builder() -> PostV1HrEmployeesRecordsCreateRequestBuilder {
        <PostV1HrEmployeesRecordsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsCreateRequestBuilder {
    employee_id: Option<String>,
    r#type: Option<PostV1HrEmployeesRecordsCreateRequestType>,
    title: Option<String>,
    institution: Option<String>,
    issued_at: Option<String>,
    valid_until: Option<String>,
    file_id: Option<String>,
    notes: Option<String>,
}

impl PostV1HrEmployeesRecordsCreateRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1HrEmployeesRecordsCreateRequestType) -> Self {
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

    pub fn issued_at(mut self, value: impl Into<String>) -> Self {
        self.issued_at = Some(value.into());
        self
    }

    pub fn valid_until(mut self, value: impl Into<String>) -> Self {
        self.valid_until = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1HrEmployeesRecordsCreateRequestBuilder::employee_id)
    /// - [`r#type`](PostV1HrEmployeesRecordsCreateRequestBuilder::r#type)
    /// - [`title`](PostV1HrEmployeesRecordsCreateRequestBuilder::title)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsCreateRequest, BuildError> {
        Ok(PostV1HrEmployeesRecordsCreateRequest {
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
