pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesRecordsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    pub r#type: PostV1HrEmployeesRecordsCreateResponseType,
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
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1HrEmployeesRecordsCreateResponse {
    pub fn builder() -> PostV1HrEmployeesRecordsCreateResponseBuilder {
        <PostV1HrEmployeesRecordsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsCreateResponseBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    r#type: Option<PostV1HrEmployeesRecordsCreateResponseType>,
    title: Option<String>,
    institution: Option<String>,
    issued_at: Option<String>,
    valid_until: Option<String>,
    file_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
}

impl PostV1HrEmployeesRecordsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PostV1HrEmployeesRecordsCreateResponseType) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesRecordsCreateResponseBuilder::id)
    /// - [`employee_id`](PostV1HrEmployeesRecordsCreateResponseBuilder::employee_id)
    /// - [`r#type`](PostV1HrEmployeesRecordsCreateResponseBuilder::r#type)
    /// - [`title`](PostV1HrEmployeesRecordsCreateResponseBuilder::title)
    /// - [`created_at`](PostV1HrEmployeesRecordsCreateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsCreateResponse, BuildError> {
        Ok(PostV1HrEmployeesRecordsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
